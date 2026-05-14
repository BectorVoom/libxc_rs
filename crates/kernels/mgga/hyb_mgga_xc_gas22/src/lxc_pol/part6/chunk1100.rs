//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1100/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1100<F: Float>(t150: F, t168: F, t177: F, t6358: F, t2109: F, t180: F, t2111: F, t746: F, t6226: F, t677: F, t136: F, t1815: F, t2153: F, t2986: F, t765: F, t215: F, t8184: F) -> (F, F, F, F, F, F, F, F, F) {
    let t20396 = 1.0 / t168 / t150;
    let t20467 = 1.0 / t6358 / t177;
    let t20475 = 1.0 / t6358 / t2109;
    let t20530 = t180 * t2111;
    let t20545 = 1.0 / t6358 / t746;
    let t20560 = t677 * t6226;
    let t20563 = t136 * t1815 * t2153;
    let t20574 = t136 * t2986 * t765;
    let t20578 = 5.0 / 108.0 * t136 * t8184 * t215;
    (t20396, t20467, t20475, t20530, t20545, t20560, t20563, t20574, t20578)
}
