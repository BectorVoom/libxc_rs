//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 996/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk996(t1986: f64, t2464: f64, t7720: f64, t75051: f64, t75060: f64, t75077: f64, t75084: f64, t69105: f64, t69107: f64, t71319: f64, t75045: f64, t75048: f64, t75054: f64, t75062: f64, t75065: f64, t75069: f64, t75072: f64, t75074: f64, t75081: f64) -> f64 {
    let t77435 = t1986 * t2464;
    let t77436 = t7720 * t77435;
    let t77437 = 0.12769379967989351819e-4_f64 * t77436;
    let t77439 = 0.5255791827870410156e-5_f64 * t75051;
    let t77441 = 0.85129199786595678799e-5_f64 * t75060;
    let t77445 = 0.16263363996404810741e-4_f64 * t75077;
    let t77447 = 0.81300399444200075499e-3_f64 * t75084;
    let t77448 = 0.72714524817717142308e-5_f64 * t75045 - t77437 - 0.58171619854173713846e-5_f64 * t75048 - t77439 - 0.17519306092901367187e-5_f64 * t75054 - t71319 - t69105 - t69107 - t77441 - 0.40878380883436523436e-5_f64 * t75062 + 0.40878380883436523436e-5_f64 * t75065 + t75069 - t75072 + 0.6505345598561924296e-5_f64 * t75074 + t77445 - 0.31062809106223861415e-2_f64 * t75081 - t77447;
    t77448
}
