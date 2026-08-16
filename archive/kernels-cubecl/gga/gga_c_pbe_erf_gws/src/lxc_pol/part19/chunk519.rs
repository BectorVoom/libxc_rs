//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 519/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk519<F: Float>(t34: F, t726: F, t93: F, t954: F, t728: F, t108: F, t2538: F, t418: F, t422: F, t532: F, t1764: F, t950: F) -> (F, F, F) {
    let t2541 = t726 * t34;
    let t2544 = t93 * t954;
    let t2547 = t728 * t34;
    let t2551 = (F::cast_from(20.0_f64) / F::cast_from(9.0_f64) * t2538 * t418 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t2541 * t532 + F::cast_from(20.0_f64) / F::cast_from(9.0_f64) * t2544 * t422 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t2547 * t532) * t108;
    let t2554 = t1764 * t950;
    (t2544, t2551, t2554)
}
