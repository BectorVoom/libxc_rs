//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 452/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk452(t481: f64, t788: f64, t2207: f64, t785: f64, t1604: f64, t2158: f64, t110: f64, t57: f64) -> (f64, f64, f64, f64) {
    let t2208 = t788 * t481;
    let t2210 = t2207 * t785 * t2208;
    let t2212 = t1604 * t2158;
    let t2214 = t57 * t110;
    (t2208, t2210, t2212, t2214)
}
