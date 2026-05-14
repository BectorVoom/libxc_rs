//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 844/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk844<F: Float>(t13736: F, t134: F, t3159: F, t7322: F, t3140: F, t347: F, t1056: F, t3143: F, t1065: F, t137: F, t420: F, t125: F, t3157: F, t2: F, t301: F, t3153: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t13737 = 0.16481111111111111111e2 * t13736;
    let t13745 = t7322 * t134 * t3159;
    let t13746 = 0.163e2 * t13745;
    let t13747 = t3140 * t134;
    let t13748 = t13747 * t347;
    let t13750 = t3143 * t1056;
    let t13754 = t3143 * t1065;
    let t13761 = t420 * t137;
    let t13768 = t125 * t3157;
    let t13771 = t3153 * t13768 * t301 * t2;
    (t13737, t13745, t13746, t13747, t13748, t13750, t13754, t13761, t13768, t13771)
}
