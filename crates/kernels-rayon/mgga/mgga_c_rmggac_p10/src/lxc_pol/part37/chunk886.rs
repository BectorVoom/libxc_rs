//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 886/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk886(t70052: f64, t14148: f64, t14150: f64, t40717: f64, t240: f64, t4738: f64, t574: f64, t7351: f64, t1614: f64, t3065: f64, t3928: f64, t13839: f64, t2044: f64, t570: f64, t7554: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t75874 = 0.19863479950205658386e-4_f64 * t70052;
    let t75876 = t14148 * t40717 * t14150;
    let t75881 = t14148 * t7351 * t574 * t240 * t4738;
    let t75886 = t3065 * t1614;
    let t75887 = t3928 * t75886;
    let t75892 = t13839 * t2044 * t7554 * t570;
    (t75874, t75876, t75881, t75886, t75887, t75892)
}
