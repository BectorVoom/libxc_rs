//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 887/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk887(t15347: f64, t69836: f64, t1653: f64, t1986: f64, t305: f64, t3141: f64, t13848: f64, t13850: f64, t8602: f64, t503: f64, t551: f64, t3157: f64) -> (f64, f64, f64, f64) {
    let t75895 = t69836 * t15347;
    let t75907 = t3141 * t1986 * t305 * t1653;
    let t75910 = t8602 * t13848 * t13850;
    let t75920 = t503 * t551;
    let t75921 = t75920 * t3157;
    (t75895, t75907, t75910, t75921)
}
