//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 552/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk552(t52: f64, t1409: f64, t78: f64, t3966: f64, t607: f64, t771: f64, t4110: f64, zeta_threshold: f64) -> f64 {
    let t150 = t52 <= zeta_threshold;
    let t4111 = t78 * t1409;
    let t4117 = piecewise3(t150, 0.0_f64, -2.0_f64 / 9.0_f64 * t4111 * t607 - 2.0_f64 / 3.0_f64 * t771 * t3966);
    let t4119 = t4110 / 2.0_f64 + t4117 / 2.0_f64;
    t4119
}
