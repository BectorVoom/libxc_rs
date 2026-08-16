//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1214/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1214(t1461: f64, t2170: f64, t573: f64, t7329: f64, t7333: f64, t7336: f64, t7696: f64, t38: f64, t4173: f64, t1497: f64, t84: f64, t77: f64) -> (f64, f64, f64, f64) {
    let t7700 = 3.0_f64 * t1461 * t2170 + t573 * t7696 + t7329 + t7333 + t7336;
    let t7702 = t4173 * t38;
    let t7705 = t84 * t1497;
    let t7706 = t77 * t7705;
    (t7700, t7702, t7705, t7706)
}
