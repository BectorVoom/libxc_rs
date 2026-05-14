//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1066/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1066<F: Float>(t683: F, t7640: F, t798: F, t9568: F, t191: F, t33828: F, t190: F, t2680: F, t305: F, t36452: F, t37991: F, t10362: F, t289: F, t9567: F, t665: F, t2344: F) -> (F, F, F, F, F, F, F, F) {
    let t43381 = t683 * t7640;
    let t43468 = t9568 * t798;
    let t43524 = t191 * t33828;
    let t43548 = 1.0 / t305 / t37991 / t190 / t2680 / t36452 / 96.0;
    let t43585 = 1.0 / t10362 / t289;
    let t43833 = t9567 * t798;
    let t43912 = t665 * t7640;
    let t43917 = t2344 * t2680;
    (t43381, t43468, t43524, t43548, t43585, t43833, t43912, t43917)
}
