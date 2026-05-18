//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 520/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk520<F: Float>(t882: F, t9090: F, t2312: F, t3130: F, t3091: F, t455: F, t145: F, t459: F, t129: F, t2276: F, t1242: F, t1232: F, t130: F) -> (F, F, F, F, F, F, F) {
    let t9092 = F::new(0.23712505529730124666e-2) * t882 * t9090;
    let t9094 = F::new(0.23712505529730124666e-2) * t2312 * t3130;
    let t9095 = t3091 * t455;
    let t9097 = t9095 * t145 * t459;
    let t9099 = t129 * t2276;
    let t9100 = t9099 * t1242;
    let t9102 = t130 * t1232;
    (t9092, t9094, t9095, t9097, t9099, t9100, t9102)
}
