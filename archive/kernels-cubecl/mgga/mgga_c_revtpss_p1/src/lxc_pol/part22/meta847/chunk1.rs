//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2986/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2986<F: Float>(t14159: F, t3964: F, t9285: F, t213: F, t225: F, t46475: F, t10019: F, t14114: F, t14145: F, t2482: F, t4114: F, t5658: F) -> (F, F, F, F) {
    let t49432 = t3964 * t14159 * t9285;
    let t49439 = t213 * t225 * t46475;
    let t49446 = t14114 * t10019;
    let t49450 = t2482 * t4114 * t5658 * t14145;
    (t49432, t49439, t49446, t49450)
}
