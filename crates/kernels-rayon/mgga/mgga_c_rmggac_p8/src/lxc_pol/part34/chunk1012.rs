//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1012/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1012(t75473: f64, t75477: f64, t75480: f64, t75484: f64, t15597: f64, t874: f64, t352: f64, t75508: f64, t75513: f64, t75517: f64, t75522: f64, t1356: f64, t69827: f64, t71502: f64, t71505: f64, t75490: f64, t75495: f64, t75500: f64, t75519: f64, t75524: f64) -> (f64, f64) {
    let t77633 = 0.7661627980793611092e-4_f64 * t75473;
    let t77634 = 0.5107751987195740728e-4_f64 * t75477;
    let t77635 = 0.2553875993597870364e-4_f64 * t75480;
    let t77636 = 0.43368970657079495308e-4_f64 * t75484;
    let t77637 = t874 * t15597;
    let t77638 = t77637 * t352;
    let t77641 = 0.86737941314158990619e-4_f64 * t75508;
    let t77642 = 0.81300399444200075499e-3_f64 * t75513;
    let t77643 = 0.54549323308490683461e-1_f64 * t75517;
    let t77646 = 0.9197635698773217773e-5_f64 * t75522;
    let t77648 = t77633 + t77634 - t77635 + t77636 - t71502 + 0.39914139006212695214e-1_f64 * t1356 * t77638 - t75490 - t75495 + t75500 + t77641 - t77642 + t77643 - 0.24527028530061914063e-5_f64 * t75519 - 0.29085809927086856923e-4_f64 * t69827 - t77646 - 0.40878380883436523436e-5_f64 * t75524 + t71505;
    (t77638, t77648)
}
