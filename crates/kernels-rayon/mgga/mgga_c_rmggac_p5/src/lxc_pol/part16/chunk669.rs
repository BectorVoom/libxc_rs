//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 669/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk669(t72: f64, t9677: f64, t2347: f64, t570: f64, t262: f64, t7204: f64, t558: f64) -> (f64, f64, f64, f64, f64) {
    let t9678 = t72 * t9677;
    let t9704 = t2347 * t570;
    let t9705 = t262 * t9704;
    let t9706 = t7204 * t9705;
    let t9708 = t2347 * t558;
    (t9678, t9704, t9705, t9706, t9708)
}
