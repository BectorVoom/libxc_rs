//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1007/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1007(t77580: f64, t637: f64, t8635: f64, t71007: f64, t75282: f64, t75285: f64, t69710: f64, t14683: f64, t38530: f64, t69722: f64, t69728: f64, t14438: f64, t2868: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t77581 = 0.6818665413561335432e-1_f64 * t77580;
    let t77582 = t637 * t8635;
    let t77583 = t71007 * t77582;
    let t77584 = 0.27274661654245341728e-1_f64 * t77583;
    let t77585 = 0.30487649791575028312e-3_f64 * t75282;
    let t77586 = 0.40911992481368012595e-1_f64 * t75285;
    let t77587 = 0.79828278012425390427e-1_f64 * t69710;
    let t77588 = t38530 * t14683;
    let t77589 = 0.42564599893297839398e-5_f64 * t77588;
    let t77590 = 0.30487649791575028312e-3_f64 * t69722;
    let t77591 = 0.30487649791575028312e-3_f64 * t69728;
    let t77592 = t2868 * t14438;
    (t77581, t77584, t77585, t77586, t77587, t77589, t77590, t77591, t77592)
}
