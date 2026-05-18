//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 487/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk487<F: Float>(t3091: F, t455: F, t145: F, t459: F, t129: F, t2276: F, t1242: F, t1232: F, t130: F, t1234: F, t136: F, t137: F) -> (F, F, F, F, F, F) {
    let t9095 = t3091 * t455;
    let t9097 = t9095 * t145 * t459;
    let t9099 = t129 * t2276;
    let t9100 = t9099 * t1242;
    let t9102 = t130 * t1232;
    let t9103 = t1234 * t136;
    let t9105 = F::new(1.0) / t137 / t9103;
    (t9095, t9097, t9099, t9100, t9102, t9105)
}
