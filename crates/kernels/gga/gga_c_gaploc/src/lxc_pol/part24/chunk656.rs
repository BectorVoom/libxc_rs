//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 656/1270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk656<F: Float>(t1242: F, t6365: F, t1233: F, t130: F, t1234: F, t137: F, t453: F, t4074: F, t4077: F, t4082: F, t4085: F, t1250: F, t2280: F, t1254: F, t864: F, t6363: F) -> (F, F, F, F, F, F, F, F) {
    let t6366 = t6365 * t1242;
    let t6368 = t130 * t1233;
    let t6371 = 1.0 / t137 / t1234 / t453;
    let t6372 = t6368 * t6371;
    let t6374 = t6372 * t4074 * t4077;
    let t6377 = t4082 * t6372 * t4085;
    let t6379 = t2280 * t1250;
    let t6381 = t864 * t1254;
    let t6383 = 189.0 / 512.0 * t6363 - 483.0 / 16384.0 * t6366 + 147.0 / 1048576.0 * t6374 - 49.0 / 1048576.0 * t6377 + 161.0 / 16384.0 * t6379 - 63.0 / 512.0 * t6381;
    (t6366, t6371, t6372, t6374, t6377, t6379, t6381, t6383)
}
