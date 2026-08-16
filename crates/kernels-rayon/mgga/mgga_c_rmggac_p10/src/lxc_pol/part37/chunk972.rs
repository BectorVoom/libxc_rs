//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 972/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk972(t69710: f64, t14683: f64, t38530: f64, t69722: f64, t69728: f64, t14438: f64, t2868: f64, t14498: f64, t5928: f64, t15526: f64, t2604: f64, t69745: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t77587 = 0.79828278012425390427e-1_f64 * t69710;
    let t77588 = t38530 * t14683;
    let t77589 = 0.42564599893297839398e-5_f64 * t77588;
    let t77590 = 0.30487649791575028312e-3_f64 * t69722;
    let t77591 = 0.30487649791575028312e-3_f64 * t69728;
    let t77592 = t2868 * t14438;
    let t77593 = 0.14967802127329760705e-1_f64 * t77592;
    let t77595 = 0.39914139006212695214e-1_f64 * t5928 * t14498;
    let t77596 = t2604 * t15526;
    let t77597 = 0.14967802127329760705e-1_f64 * t77596;
    let t77598 = 0.16263363996404810741e-4_f64 * t69745;
    (t77587, t77589, t77590, t77591, t77593, t77595, t77597, t77598)
}
