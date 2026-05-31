//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1345/1427 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1345<F: Float>(t10086: F, t1125: F, t2469: F, t12282: F, t36095: F, t36098: F, t36100: F, t36103: F, t36105: F, t36108: F, t36109: F, t36111: F, t36113: F, t36116: F, t36127: F, t36130: F, t36252: F, t36255: F, t36259: F, t3846: F, t7053: F, t7056: F) -> F {
    let t36262 = F::cast_from(2.0_f64) * t2469 * t1125 * t10086;
    let t36263 = F::cast_from(4.0_f64) * t12282 * t7056 - t3846 * t7053 + t36095 + t36098 + t36100 + t36103 - t36105 + t36108 + t36109 - t36111 + t36113 + t36116 - t36127 + t36130 + t36252 - t36255 + t36259 + t36262;
    t36263
}
