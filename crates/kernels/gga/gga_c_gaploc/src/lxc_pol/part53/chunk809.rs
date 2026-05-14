//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 809/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk809<F: Float>(t1445: F, t25405: F, t3209: F, t5748: F, t13034: F, t15751: F, t41448: F, t41451: F, t41454: F, t41457: F, t41466: F, t41469: F, t41474: F, t41477: F, t11807: F, t3277: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t44142 = 0.27606906686822939767e2 * t5748 * t1445 * t25405 * t3209;
    let t44144 = 0.27606906686822939767e2 * t15751 * t13034;
    let t44147 = 0.31952438294933958063e0 * t41448;
    let t44148 = 0.89376224879626066674e-1 * t41451;
    let t44149 = 0.59584149919750711116e-1 * t41454;
    let t44150 = 0.15337170381568299871e1 * t41457;
    let t44154 = 0.29792074959875355558e-1 * t41466;
    let t44155 = 0.29792074959875355558e-1 * t41469;
    let t44156 = 0.25561950635947166451e1 * t41474;
    let t44157 = 0.12780975317973583225e0 * t41477;
    let t44162 = 0.25025342966295298669e1 * t3277 * t11807;
    (t44142, t44144, t44147, t44148, t44149, t44150, t44154, t44155, t44156, t44157, t44162)
}
