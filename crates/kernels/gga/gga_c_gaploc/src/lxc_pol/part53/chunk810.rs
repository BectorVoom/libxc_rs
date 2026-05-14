//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 810/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk810<F: Float>(t10057: F, t13045: F, t13013: F, t5782: F, t1445: F, t2087: F, t3234: F, t8483: F, t3009: F, t9688: F, t41512: F, t41515: F, t41518: F, t41538: F, t41542: F, t10105: F, t2969: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t44164 = 0.25025342966295298669e1 * t10057 * t13045;
    let t44170 = 0.69017266717057349418e1 * t5782 * t13013;
    let t44174 = 0.69017266717057349418e1 * t2087 * t1445 * t8483 * t3234;
    let t44178 = 0.69017266717057349418e1 * t2087 * t1445 * t3009 * t9688;
    let t44179 = 0.17875244975925213335e0 * t41512;
    let t44180 = 0.29792074959875355558e-1 * t41515;
    let t44181 = 0.59584149919750711116e-1 * t41518;
    let t44185 = 0.17041300423964777634e0 * t41538;
    let t44186 = 0.25561950635947166451e0 * t41542;
    let t44194 = t2969 * t10105;
    (t44164, t44170, t44174, t44178, t44179, t44180, t44181, t44185, t44186, t44194)
}
