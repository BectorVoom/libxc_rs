//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1209/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1209(t3: f64, t7690: f64, t1461: f64, t2170: f64, t573: f64, t7329: f64, t7333: f64, t7336: f64, t38: f64, t4173: f64, t1497: f64, t84: f64, param_d: f64) -> (f64, f64, f64, f64, f64) {
    let t7691 = t3 * t7690;
    let t7696 = param_d * t7690;
    let t7700 = 3.0_f64 * t1461 * t2170 + t573 * t7696 + t7329 + t7333 + t7336;
    let t7702 = t4173 * t38;
    let t7705 = t84 * t1497;
    (t7691, t7696, t7700, t7702, t7705)
}
