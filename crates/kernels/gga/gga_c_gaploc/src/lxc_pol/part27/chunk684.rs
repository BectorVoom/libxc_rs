//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 684/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk684<F: Float>(t145: F, t459: F, t6361: F, t1232: F, t1236: F, t1242: F, t1233: F, t130: F, t1234: F, t137: F, t453: F, t4074: F, t4077: F) -> (F, F, F, F, F, F) {
    let t6363 = t6361 * t145 * t459;
    let t6365 = t1232 * t1236;
    let t6366 = t6365 * t1242;
    let t6368 = t130 * t1233;
    let t6371 = F::new(1.0) / t137 / t1234 / t453;
    let t6372 = t6368 * t6371;
    let t6374 = t6372 * t4074 * t4077;
    (t6363, t6365, t6366, t6371, t6372, t6374)
}
