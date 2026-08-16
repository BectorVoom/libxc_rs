//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 909/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk909<F: Float>(t1065: F, t3143: F, t137: F, t420: F, t125: F, t3157: F, t2: F, t301: F, t3153: F, t1149: F, t986: F, t1152: F) -> (F, F, F, F, F, F) {
    let t13754 = t3143 * t1065;
    let t13761 = t420 * t137;
    let t13768 = t125 * t3157;
    let t13771 = t3153 * t13768 * t301 * t2;
    let t13787 = t986 * t1149;
    let t13788 = t13787 * t1152;
    (t13754, t13761, t13768, t13771, t13787, t13788)
}
