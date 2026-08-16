//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 463/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk463(t2102: f64, t2105: f64, t265: f64, t254: f64, t118: f64, t510: f64, t116: f64) -> (f64, f64, f64, f64) {
    let t2106 = t2102 * t265 * t2105;
    let t2108 = 0.63479958930231934629e-2_f64 * t254 * t2106;
    let t2110 = 1.0_f64 / t510 / t118;
    let t2111 = t116 * t2110;
    (t2106, t2108, t2110, t2111)
}
