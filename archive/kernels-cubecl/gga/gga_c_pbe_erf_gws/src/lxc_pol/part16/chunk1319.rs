//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1319/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1319<F: Float>(t353: F, t55151: F, t859: F, t938: F, t53424: F, t14188: F, t14881: F, t14888: F, t14922: F, t19895: F, t22379: F, t2408: F, t2409: F, t4385: F, t52263: F, t52266: F, t52270: F, t53378: F, t53386: F, t53395: F, t55137: F, t55142: F, t55145: F, t6781: F, t6793: F, t827: F, t9218: F, t9283: F) -> F {
    let t55154 = t859 * t353 * t55151 * t938;
    let t55161 = F::cast_from(35.0_f64) / F::cast_from(288.0_f64) * t53424;
    let t55162 = t53378 / F::cast_from(384.0_f64) + F::cast_from(7.0_f64) / F::cast_from(48.0_f64) * t52263 + t2408 * t2409 * t6781 * t14922 / F::cast_from(24.0_f64) + t53386 / F::cast_from(12.0_f64) + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t52266 + F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t52270 - t53395 / F::cast_from(384.0_f64) - t4385 * t55137 / F::cast_from(48.0_f64) - t827 * t55142 / F::cast_from(48.0_f64) - F::cast_from(35.0_f64) / F::cast_from(432.0_f64) * t55145 + t2408 * t9283 * t14881 * t9218 / F::cast_from(8.0_f64) + t6793 * t55154 / F::cast_from(24.0_f64) + t19895 * t14888 / F::cast_from(48.0_f64) + t22379 * t14188 / F::cast_from(24.0_f64) - t55161;
    t55162
}
