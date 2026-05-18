//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 944/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk944<F: Float>(t190: F, t8448: F, t1: F, t116: F, t3703: F, t612: F, t144: F, t3137: F, t674: F, t5059: F, t641: F, t1908: F, t198: F) -> (F, F, F, F, F, F, F, F) {
    let t11532 = t190 * t8448;
    let t11533 = t11532 * t1;
    let t11534 = t116 * t11533;
    let t11535 = t11534 * t3703;
    let t11537 = t116 * t612;
    let t11539 = t3137 * t144 * t674;
    let t11540 = t11539 * t5059;
    let t11541 = t11537 * t11540;
    let t11543 = t116 * t641;
    let t11546 = t3137 * t198 * t1908 * t5059;
    (t11533, t11534, t11535, t11537, t11540, t11541, t11543, t11546)
}
