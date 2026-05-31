//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 818/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk818<F: Float>(t2402: F, t338: F, t892: F, t2373: F, t2384: F, t2401: F, t2408: F, t335: F, t6130: F, t6135: F, t6140: F, t6145: F, t6151: F, t6156: F, t6160: F, t6164: F, t6170: F, t6173: F, t6175: F, t6726: F, t6731: F, t6741: F, t6746: F, t6748: F, t827: F) -> (F, F) {
    let t6751 = t338 * t892 * t2402;
    let t6754 = -t335 * t6130 / F::cast_from(16.0_f64) - t827 * t6135 / F::cast_from(8.0_f64) - t2408 * t6140 / F::cast_from(8.0_f64) + t827 * t6145 / F::cast_from(16.0_f64) + F::cast_from(3.0_f64) / F::cast_from(16.0_f64) * t827 * t6151 + F::cast_from(7.0_f64) / F::cast_from(96.0_f64) * t6156 + t6160 * t6164 / F::cast_from(48.0_f64) - t2384 * t2373 / F::cast_from(16.0_f64) - t335 * t6170 / F::cast_from(32.0_f64) - F::cast_from(7.0_f64) / F::cast_from(48.0_f64) * t6173 + F::cast_from(7.0_f64) / F::cast_from(96.0_f64) * t6175 - t335 * t6726 / F::cast_from(96.0_f64) - t6731 + t335 * t6741 / F::cast_from(96.0_f64) - F::cast_from(7.0_f64) / F::cast_from(96.0_f64) * t6746 - F::cast_from(7.0_f64) / F::cast_from(48.0_f64) * t6748 + F::cast_from(3.0_f64) / F::cast_from(16.0_f64) * t2401 * t6751;
    (t6751, t6754)
}
