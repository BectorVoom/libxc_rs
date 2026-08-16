//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 971/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk971(t77572: f64, t15504: f64, t16043: f64, t637: f64, t8625: f64, t71163: f64, t8631: f64, t72142: f64, t8635: f64, t71007: f64, t75282: f64, t75285: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t77573 = 0.53205749866622299248e-5_f64 * t77572;
    let t77574 = t16043 * t15504;
    let t77575 = 0.42564599893297839398e-5_f64 * t77574;
    let t77576 = t637 * t8625;
    let t77577 = t71163 * t77576;
    let t77578 = 0.40911992481368012592e-1_f64 * t77577;
    let t77579 = t637 * t8631;
    let t77580 = t72142 * t77579;
    let t77581 = 0.6818665413561335432e-1_f64 * t77580;
    let t77582 = t637 * t8635;
    let t77583 = t71007 * t77582;
    let t77584 = 0.27274661654245341728e-1_f64 * t77583;
    let t77585 = 0.30487649791575028312e-3_f64 * t75282;
    let t77586 = 0.40911992481368012595e-1_f64 * t75285;
    (t77573, t77575, t77578, t77581, t77584, t77585, t77586)
}
