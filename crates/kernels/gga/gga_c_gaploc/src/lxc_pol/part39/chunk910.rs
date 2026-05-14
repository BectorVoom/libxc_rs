//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 910/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk910<F: Float>(t1445: F, t2087: F, t3234: F, t8483: F, t3009: F, t9688: F, t41512: F, t41515: F, t41518: F, t41538: F, t41542: F, t10105: F, t2969: F, t11127: F, t7324: F, t3511: F, t7822: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t44174 = 0.69017266717057349418e1 * t2087 * t1445 * t8483 * t3234;
    let t44178 = 0.69017266717057349418e1 * t2087 * t1445 * t3009 * t9688;
    let t44179 = 0.17875244975925213335e0 * t41512;
    let t44180 = 0.29792074959875355558e-1 * t41515;
    let t44181 = 0.59584149919750711116e-1 * t41518;
    let t44185 = 0.17041300423964777634e0 * t41538;
    let t44186 = 0.25561950635947166451e0 * t41542;
    let t44194 = t2969 * t10105;
    let t44196 = t7324 * t11127;
    let t44198 = t7822 * t3511;
    (t44174, t44178, t44179, t44180, t44181, t44185, t44186, t44194, t44196, t44198)
}
