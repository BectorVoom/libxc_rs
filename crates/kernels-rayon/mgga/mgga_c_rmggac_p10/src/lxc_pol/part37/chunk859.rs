//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 859/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk859(t325: f64, t551: f64, t13897: f64, t15098: f64, t30526: f64, t1326: f64, t75399: f64, t13916: f64, t13928: f64, t1612: f64, t11704: f64, t13931: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t75411 = t551 * t325;
    let t75412 = t75411 * t13897;
    let t75414 = t30526 * t15098;
    let t75416 = t1326 * t75399;
    let t75417 = t13916 * t75416;
    let t75419 = t13928 * t1612;
    let t75421 = t13931 * t11704;
    (t75411, t75412, t75414, t75416, t75417, t75419, t75421)
}
