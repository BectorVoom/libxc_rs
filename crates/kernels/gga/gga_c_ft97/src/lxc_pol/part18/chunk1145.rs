//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1145/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1145<F: Float>(t100186: F, t446: F, t7824: F, t1651: F, t6469: F, t3103: F, t5617: F, t1317: F, t1800: F, t28: F, t22862: F, t469: F, t5665: F, t965: F, t1808: F, t6454: F) -> (F, F, F, F, F, F, F) {
    let t100188 = t446 * t7824 * t100186;
    let t100190 = t6469 * t1651;
    let t100192 = t446 * t7824 * t100190;
    let t100195 = t5617 * t3103;
    let t100198 = t1317 * t28 * t1800 * t100195;
    let t100203 = t5665 * t28 * t469 * t22862 * t965;
    let t100208 = t5665 * t28 * t469 * t6454 * t1808;
    (t100188, t100190, t100192, t100195, t100198, t100203, t100208)
}
