//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1069/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1069<F: Float>(t12024: F, t906: F, t11819: F, t8599: F, t2168: F, t11990: F, t4386: F, t2127: F, t3781: F, t850: F, t860: F, t2142: F, t3788: F) -> (F, F, F, F, F, F) {
    let t12025 = t12024 * t906;
    let t12029 = t8599 * t11819;
    let t12031 = t2168 * t12029 / F::cast_from(8.0_f64);
    let t12032 = t4386 * t11990;
    let t12034 = t2168 * t12032 / F::cast_from(24.0_f64);
    let t12036 = t850 * t3781 * t2127;
    let t12038 = t12036 * t860 / F::cast_from(96.0_f64);
    let t12039 = t3788 * t2142;
    (t12025, t12031, t12034, t12036, t12038, t12039)
}
