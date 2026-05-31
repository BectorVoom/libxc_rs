//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1326/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1326<F: Float>(t3123: F, t9127: F, t11548: F, t14007: F, t12015: F, t14031: F, t11501: F, t14567: F, t6608: F, t55486: F, t56954: F, t56956: F, t56958: F, t56960: F, t56962: F, t56964: F, t56966: F) -> F {
    let t56968 = t3123 * t9127;
    let t56970 = t14007 * t11548;
    let t56972 = t14031 * t12015;
    let t56975 = t6608 * t11501 * t14567;
    let t56977 = t56954 / F::cast_from(24.0_f64) - t56956 / F::cast_from(48.0_f64) + t56958 / F::cast_from(128.0_f64) - t56960 / F::cast_from(48.0_f64) + t56962 / F::cast_from(96.0_f64) - t56964 / F::cast_from(384.0_f64) + F::cast_from(5.0_f64) / F::cast_from(96.0_f64) * t56966 + t56968 / F::cast_from(24.0_f64) + t56970 / F::cast_from(384.0_f64) - t56972 / F::cast_from(384.0_f64) + t56975 / F::cast_from(96.0_f64) + t55486;
    t56977
}
