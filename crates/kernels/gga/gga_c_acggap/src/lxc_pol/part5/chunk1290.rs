//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1290/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1290<F: Float>(t1856: F, t3237: F, t1008: F, t6200: F, t1095: F, t322: F, t384: F, t398: F, t5674: F, t1165: F, t4282: F, t5249: F, t530: F) -> (F, F, F, F) {
    let t23944 = t3237 * t1856;
    let t23946 = t1008 * t6200;
    let t23951 = t384 * t398 * t1095 * t5674 * t322;
    let t23959 = t4282 * t1165 * t530 * t5249;
    (t23944, t23946, t23951, t23959)
}
