//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1082/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1082(t530: f64, t73395: f64, t73397: f64, t75607: f64, t75611: f64, t75623: f64, t77686: f64, t77690: f64, t77691: f64, t77693: f64, t77694: f64, t77695: f64, t77696: f64, t77697: f64, t77700: f64, t77703: f64, t77704: f64, t77705: f64) -> f64 {
    let t80275 = -t77686 - t75607 - 0.17451485956252114153e-4_f64 * t75611 - t77690 - t77691 - 0.17519306092901367186e-5_f64 * t75623 + t77693 - t77694 + t77695 - t77696 + t77697 + t77700 + t73395 + t77703 - 0.2363e1_f64 * t530 * t73397 + t77704 + t77705;
    t80275
}
