//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 772/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk772<F: Float>(t742: F, t85: F, t776: F, t2429: F, t2493: F, t2484: F, t2527: F, t752: F, t2718: F, t873: F, t872: F, t206: F) -> (F, F, F, F, F, F, F, F) {
    let t8931 = t85 * t742;
    let t8932 = t8931 * t776;
    let t8934 = t2429 * t2493;
    let t8936 = t2484 * t2527;
    let t8937 = t752 * t8936;
    let t8939 = t2718 * t873;
    let t8942 = t872 * t872;
    let t8943 = F::cast_from(1.0_f64) / t8942;
    let t8944 = t206 * t8943;
    (t8931, t8932, t8934, t8937, t8939, t8942, t8943, t8944)
}
