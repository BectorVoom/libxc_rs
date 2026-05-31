//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 261/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk261<F: Float>(t370: F, t942: F, t27: F, t89: F, t354: F, t923: F, t348: F, t110: F, t447: F, t925: F, t452: F, t464: F, t920: F) -> (F, F, F, F, F, F, F) {
    let t943 = t370 * t942;
    let t945 = t89 * t27 * t943;
    let t947 = -t354 - t923 / F::cast_from(18.0_f64) - t945 / F::cast_from(6.0_f64);
    let t948 = t348 * t947;
    let t951 = t447 * t110 * t925;
    let t955 = t452 * t110 * t942;
    let t958 = t464 * t920;
    (t943, t945, t947, t948, t951, t955, t958)
}
