//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 894/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk894<F: Float>(t1105: F, t2416: F, t3068: F, t9283: F, t2362: F, t2397: F, t2408: F, t2498: F, t2503: F, t3052: F, t3733: F, t3921: F, t6778: F, t827: F, t8629: F, t8654: F, t8671: F, t8677: F, t8790: F, t8793: F, t9726: F, t9729: F, t9899: F, t9902: F, t9907: F, t9912: F, t9917: F, t9923: F, t9928: F, t9932: F) -> (F, F) {
    let t9941 = t2416 * t1105;
    let t9942 = t9941 * t3068;
    let t9943 = t9283 * t9942;
    let t9946 = -t8671 - t9726 * t3733 / F::cast_from(96.0_f64) - t827 * t9899 / F::cast_from(96.0_f64) - t9902 * t2362 / F::cast_from(48.0_f64) - t9729 * t3733 / F::cast_from(96.0_f64) + t9907 * t6778 / F::cast_from(48.0_f64) + t8677 + t2498 * t2503 / F::cast_from(48.0_f64) + F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t9912 + t8629 * t9917 / F::cast_from(48.0_f64) + t8629 * t9923 / F::cast_from(96.0_f64) + t2408 * t9928 / F::cast_from(24.0_f64) + t2408 * t9932 / F::cast_from(24.0_f64) + t8793 * t8790 / F::cast_from(24.0_f64) + t3921 * t2397 / F::cast_from(96.0_f64) - t8654 * t3052 / F::cast_from(24.0_f64) - t2408 * t9943 / F::cast_from(12.0_f64);
    (t9942, t9946)
}
