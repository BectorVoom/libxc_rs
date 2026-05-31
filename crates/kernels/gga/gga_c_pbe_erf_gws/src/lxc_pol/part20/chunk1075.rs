//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1075/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1075<F: Float>(t12074: F, t12076: F, t11478: F, t3139: F, t3140: F, t3138: F, t875: F, t2168: F, t11994: F, t2255: F, t2279: F, t3820: F, t6484: F) -> (F, F, F, F, F, F, F) {
    let t12078 = t12074 * t12076 / F::cast_from(96.0_f64);
    let t12080 = t3139 * t11478 * t3140;
    let t12082 = t3138 * t12080 / F::cast_from(16.0_f64);
    let t12084 = t3139 * t11478 * t875;
    let t12086 = t2168 * t12084 / F::cast_from(96.0_f64);
    let t12088 = t2255 * t11994 * t2279;
    let t12092 = t6484 * t3820;
    (t12078, t12080, t12082, t12084, t12086, t12088, t12092)
}
