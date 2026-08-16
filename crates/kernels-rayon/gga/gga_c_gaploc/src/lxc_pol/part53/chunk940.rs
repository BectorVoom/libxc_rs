//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 940/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk940(t13034: f64, t15751: f64, t41448: f64, t41451: f64, t41454: f64, t41457: f64, t41466: f64, t41469: f64, t41474: f64, t41477: f64, t11807: f64, t3277: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t44144 = 0.27606906686822939767e2_f64 * t15751 * t13034;
    let t44147 = 0.31952438294933958063e0_f64 * t41448;
    let t44148 = 0.89376224879626066674e-1_f64 * t41451;
    let t44149 = 0.59584149919750711116e-1_f64 * t41454;
    let t44150 = 0.15337170381568299871e1_f64 * t41457;
    let t44154 = 0.29792074959875355558e-1_f64 * t41466;
    let t44155 = 0.29792074959875355558e-1_f64 * t41469;
    let t44156 = 0.25561950635947166451e1_f64 * t41474;
    let t44157 = 0.12780975317973583225e0_f64 * t41477;
    let t44162 = 0.25025342966295298669e1_f64 * t3277 * t11807;
    (t44144, t44147, t44148, t44149, t44150, t44154, t44155, t44156, t44157, t44162)
}
