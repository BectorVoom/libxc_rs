//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1016/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1016(t14469: f64, t2868: f64, t75615: f64, t75620: f64, t75626: f64, t75629: f64, t75632: f64, t75635: f64, t69907: f64, t14584: f64, t623: f64, t2141: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t77685 = t2868 * t14469;
    let t77686 = 0.39914139006212695213e-1_f64 * t77685;
    let t77690 = 0.23268647941669485538e-4_f64 * t75615;
    let t77691 = 0.3941843870902807617e-5_f64 * t75620;
    let t77693 = 0.5255791827870410156e-5_f64 * t75626;
    let t77694 = 0.1276937996798935182e-4_f64 * t75629;
    let t77695 = 0.2553875993597870364e-4_f64 * t75632;
    let t77696 = 0.3830813990396805546e-4_f64 * t75635;
    let t77697 = 0.18183107769496894487e-1_f64 * t69907;
    let t77698 = t623 * t14584;
    let t77699 = t77698 * t2141;
    (t77686, t77690, t77691, t77693, t77694, t77695, t77696, t77697, t77699)
}
