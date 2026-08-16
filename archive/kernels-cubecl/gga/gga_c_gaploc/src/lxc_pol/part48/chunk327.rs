//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 327/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk327<F: Float>(t747: F, t952: F, t841: F, t977: F, t1628: F, t973: F, t2027: F, t959: F, t701: F, t733: F, t2365: F, t2022: F) -> (F, F, F, F, F) {
    let t2592 = t952 * t747;
    let t2595 = t977 * t841;
    let t2598 = t1628 * t973;
    let t2601 = t2027 * t959;
    let t2603 = t733 * t701;
    let t2604 = t2365 * t2603;
    let t2605 = t2022 * t2604;
    (t2592, t2595, t2598, t2601, t2605)
}
