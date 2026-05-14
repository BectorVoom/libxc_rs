//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 488/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk488<F: Float>(t123: F, t9078: F, t4385: F, t1365: F, t6520: F, t6525: F, t6417: F, t883: F, t2325: F, t882: F, t2321: F, t2440: F, t2312: F, t3130: F, t3091: F, t455: F) -> (F, F, F, F, F, F, F, F) {
    let t9079 = t9078 * t123;
    let t9080 = t9079 * t4385;
    let t9083 = t1365 * t6520;
    let t9085 = 0.23712505529730124666e-2 * t6525 * t9083;
    let t9086 = t883 * t6417;
    let t9087 = t2325 * t9086;
    let t9089 = 0.23712505529730124666e-2 * t882 * t9087;
    let t9090 = t2440 * t2321;
    let t9092 = 0.23712505529730124666e-2 * t882 * t9090;
    let t9094 = 0.23712505529730124666e-2 * t2312 * t3130;
    let t9095 = t3091 * t455;
    (t9079, t9080, t9085, t9086, t9089, t9092, t9094, t9095)
}
