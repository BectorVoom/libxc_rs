//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1077/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1077<F: Float>(t19384: F, t127: F, t1563: F, t16423: F, t19083: F, t19268: F, t19349: F, t19351: F, t19355: F, t19357: F, t19359: F, t19362: F, t19365: F, t19367: F, t19373: F, t19381: F, t506: F) -> (F, F) {
    let t19385 = F::cast_from(0.77947333333333333333e1_f64) * t19384;
    let t19386 = -t19349 + t19351 + t19355 + t19357 + t19359 + t19362 + t19365 + F::cast_from(0.1762848e3_f64) * t127 * t19367 * t19268 - t19373 - F::cast_from(0.146904e1_f64) * t127 * t506 * t19083 + F::cast_from(0.1762848e2_f64) * t127 * t1563 * t16423 - F::cast_from(6.0_f64) * t19381 + t19385;
    (t19385, t19386)
}
