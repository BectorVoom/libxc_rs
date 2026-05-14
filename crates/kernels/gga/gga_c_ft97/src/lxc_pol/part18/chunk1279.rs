//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1279/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1279<F: Float>(t26580: F, t92: F, t24073: F, t6580: F, t12605: F, t95403: F, t2178: F, t6685: F, t2180: F, t2142: F, t27191: F, t1053: F, t95026: F, t12664: F, t23541: F, t23536: F) -> (F, F, F, F, F, F, F, F) {
    let t104446 = t26580 * t92;
    let t104450 = 2.0 / 9.0 * t6580 * t24073;
    let t104453 = t95403 * t12605;
    let t104462 = t6685 * t2178;
    let t104463 = t104462 * t2180;
    let t104465 = t2142 * t27191;
    let t104467 = t95026 * t1053;
    let t104469 = t12664 * t23541;
    let t104471 = t12664 * t23536;
    (t104446, t104450, t104453, t104463, t104465, t104467, t104469, t104471)
}
