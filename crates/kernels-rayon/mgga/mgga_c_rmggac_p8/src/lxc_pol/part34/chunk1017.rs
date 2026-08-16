//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1017/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1017(t77699: f64, t75638: f64, t75640: f64, t75644: f64, t14501: f64, t1540: f64, t2868: f64, t3230: f64, t75607: f64, t75611: f64, t75623: f64, t77686: f64, t77690: f64, t77691: f64, t77693: f64, t77694: f64, t77695: f64, t77696: f64, t77697: f64) -> f64 {
    let t77700 = 0.13637330827122670864e-1_f64 * t77699;
    let t77703 = 0.14967802127329760705e-1_f64 * t75638;
    let t77704 = 0.10227998120342003148e-1_f64 * t75640;
    let t77705 = 0.10227998120342003148e-1_f64 * t75644;
    let t77706 = -t77686 - 0.59871208509319042821e-1_f64 * t2868 * t14501 - t75607 - 0.17451485956252114154e-4_f64 * t75611 - t77690 - t77691 - 0.17519306092901367187e-5_f64 * t75623 + t77693 - t77694 + t77695 - t77696 + t77697 + t77700 - 0.19957069503106347607e-1_f64 * t1540 * t3230 + t77703 + t77704 + t77705;
    t77706
}
