//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1114/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1114<F: Float>(t263: F, t9570: F, t1403: F, t27952: F, t681: F, t24178: F, t6745: F, t27938: F, t24237: F, t28002: F, t11176: F, t1402: F, t28012: F, t24543: F, t27754: F, t27758: F) -> (F, F, F, F, F, F, F, F, F) {
    let t107920 = t263 * t9570;
    let t107943 = 2.0 / 9.0 * t1403 * t681 * t27952;
    let t107945 = 2.0 / 9.0 * t6745 * t24178;
    let t107958 = t1403 * t681 * t27938 / 9.0;
    let t107971 = t24237 * t28002 / 27.0;
    let t107979 = t1402 * t11176 * t28012;
    let t107996 = t24543 * t27754;
    let t107997 = 2.0 / 9.0 * t107996;
    let t107998 = t24543 * t27758;
    (t107920, t107943, t107945, t107958, t107971, t107979, t107996, t107997, t107998)
}
