//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1076/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1076(t75252: f64, t69667: f64, t73344: f64, t75248: f64, t75266: f64, t75269: f64, t77529: f64, t77533: f64, t77537: f64, t77540: f64, t77542: f64, t77545: f64, t77550: f64, t77553: f64, t77556: f64, t77557: f64, t77558: f64) -> f64 {
    let t80244 = 0.13469175824740901074e-6_f64 * t75252;
    let t80247 = -t69667 - t73344 + t75248 + t77529 + t77533 + t77537 + t77540 + t80244 + t77542 + t77545 + 0.58171619854173713844e-5_f64 * t75266 - 0.58171619854173713844e-5_f64 * t75269 + t77550 - t77553 - t77556 - t77557 - t77558;
    t80247
}
