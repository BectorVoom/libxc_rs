//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 261/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk261<F: Float>(t24: F, t287: F, t321: F, t320: F, t146: F, t299: F, t283: F, t284: F) -> (F, F, F) {
    let t924 = t24 * t287;
    let t925 = t321 * t924;
    let t927 = F::cast_from(0.28977204965962526182e-1_f64) * t320 * t925;
    let t928 = t146 * t299;
    let t929 = t283 * t284;
    let t930 = t928 * t929;
    (t924, t927, t930)
}
