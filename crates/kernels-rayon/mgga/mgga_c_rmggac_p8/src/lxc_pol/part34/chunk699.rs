//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 699/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk699(t14302: f64, t69212: f64, t14301: f64, t25529: f64, t14305: f64, t69240: f64, t1322: f64, t838: f64, t874: f64, t25525: f64, t3065: f64, t14327: f64, t3814: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t69419 = t14302 * t69212;
    let t69421 = t25529 * t14301;
    let t69424 = t14305 * t69240;
    let t69428 = t838 * t874 * t1322;
    let t69433 = t25525 * t3065;
    let t69436 = t3814 * t14327;
    (t69419, t69421, t69424, t69428, t69433, t69436)
}
