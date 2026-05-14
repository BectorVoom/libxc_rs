//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1208/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1208<F: Float>(t1616: F, t2011: F, t3909: F, t36095: F, t36100: F, t36103: F, t36105: F, t36109: F, t36111: F, t36113: F, t36116: F, t36119: F, t36270: F, t36271: F, t36275: F, t36283: F, t36285: F, t36288: F, t38537: F, t38556: F, t38689: F) -> (F, F) {
    let t38692 = 2.0 * t1616 * t3909 * t2011;
    let t38693 = -t36095 + t38537 - t36100 - t36103 + t36105 + t38556 - t38689 - t36109 + t36111 - t36113 - t36116 + t36119 + t38692 - t36270 - t36271 - t36275 + t36283 - t36285 + t36288;
    (t38692, t38693)
}
