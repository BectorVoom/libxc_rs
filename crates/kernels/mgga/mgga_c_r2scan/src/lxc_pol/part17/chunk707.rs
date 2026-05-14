//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 707/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk707<F: Float>(t1591: F, t2168: F, t1541: F, t545: F, t548: F, t110: F, t6189: F, t6188: F, t6072: F, t2183: F, t2097: F, t547: F, t546: F, t560: F, t6212: F, t6211: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t6425 = t1591 * t2168;
    let t6448 = t545 * t1541;
    let t6449 = t6448 * t548;
    let t6461 = t6189 * t110;
    let t6462 = t6188 * t6461;
    let t6463 = t6462 * t6072;
    let t6465 = t2183 * t2168;
    let t6474 = t547 * t2097;
    let t6475 = t546 * t6474;
    let t6476 = t6212 * t560;
    let t6477 = t6211 * t6476;
    (t6425, t6448, t6449, t6461, t6462, t6463, t6465, t6474, t6475, t6476, t6477)
}
