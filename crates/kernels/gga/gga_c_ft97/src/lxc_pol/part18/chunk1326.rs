//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1326/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1326<F: Float>(t1360: F, t3000: F, t358: F, t89: F, t92: F, t95262: F, t1039: F, t2087: F, t574: F, t5900: F, t6662: F, t95099: F, t3408: F, t5842: F, t1369: F, t2112: F, t28: F) -> (F, F, F, F, F, F) {
    let t105590 = t89 * t3000 * t1360 * t358;
    let t105592 = t95262 * t92;
    let t105596 = t105592 * t574 * t5900 * t1039 * t2087;
    let t105598 = t95099 * t6662;
    let t105599 = 2.0 / 27.0 * t105598;
    let t105600 = t5842 * t3408;
    let t105603 = t1369 * t28 * t2112 * t105600;
    (t105590, t105596, t105598, t105599, t105600, t105603)
}
