//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1026/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1026(t15606: f64, t275: f64, t71594: f64, t14441: f64, t5928: f64, t75736: f64, t75739: f64, t1550: f64, t2228: f64, t2347: f64, t1624: f64, t3204: f64, t71589: f64, t71607: f64, t739: f64, t75733: f64, t77803: f64, t77804: f64, t77807: f64, t77810: f64, t77812: f64, t77816: f64) -> f64 {
    let t77819 = t275 * t15606;
    let t77820 = 0.15243824895787514157e-3_f64 * t71594;
    let t77823 = 0.39914139006212695214e-1_f64 * t5928 * t14441;
    let t77824 = 0.10909864661698136691e0_f64 * t75736;
    let t77825 = 0.21819729323396273382e0_f64 * t75739;
    let t77827 = t1550 * t2228 * t2347;
    let t77828 = 0.2993560425465952141e-1_f64 * t77827;
    let t77829 = t77803 - t77804 + t77807 + t77810 + t77812 + t71589 - 0.11974241701863808564e0_f64 * t1550 * t3204 * t1624 - 0.59871208509319042821e-1_f64 * t739 * t77816 + t77819 - t77820 + 0.29085809927086856923e-4_f64 * t75733 + t77823 + t77824 - t77825 + t77828 + t71607;
    t77829
}
