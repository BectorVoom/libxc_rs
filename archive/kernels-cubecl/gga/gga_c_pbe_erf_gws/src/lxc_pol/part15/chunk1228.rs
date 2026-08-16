//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1228/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1228<F: Float>(t13772: F, t14397: F, t14437: F, t14791: F, t2388: F, t2392: F, t2408: F, t3040: F, t50927: F, t52940: F, t52944: F, t52952: F, t52956: F, t52959: F, t52962: F, t52969: F, t52971: F, t52973: F, t52976: F, t9218: F, t9283: F) -> F {
    let t52978 = -t2388 * t14437 / F::cast_from(96.0_f64) + t52940 / F::cast_from(384.0_f64) + t52944 / F::cast_from(768.0_f64) + t2408 * t9283 * t14791 * t9218 / F::cast_from(8.0_f64) - t52952 / F::cast_from(3072.0_f64) + t52956 / F::cast_from(768.0_f64) - t52959 / F::cast_from(192.0_f64) - t52962 + F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t50927 - t3040 * t13772 / F::cast_from(48.0_f64) - t2392 * t14397 / F::cast_from(96.0_f64) + t52969 + t52971 + t52973 + t52976 / F::cast_from(768.0_f64);
    t52978
}
