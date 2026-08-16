//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1188/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1188(t1873: f64, t24932: f64, t27888: f64, t6534: f64, t7266: f64, t31227: f64, t31229: f64, t31231: f64, t31233: f64, t31235: f64, t31237: f64, t31239: f64, t31877: f64, t31880: f64, t671: f64, t8446: f64) -> f64 {
    let t31883 = t24932 * t1873;
    let t31885 = t27888 * t1873;
    let t31887 = t7266 * t6534;
    let t31892 = 2.0_f64 * t31880 * t671 + 2.0_f64 * t31227 + 2.0_f64 * t31229 + 2.0_f64 * t31231 + t31233 + t31235 + t31237 + t31239 + t31877 + 2.0_f64 * t31883 + 2.0_f64 * t31885 + 2.0_f64 * t31887 + t8446;
    t31892
}
