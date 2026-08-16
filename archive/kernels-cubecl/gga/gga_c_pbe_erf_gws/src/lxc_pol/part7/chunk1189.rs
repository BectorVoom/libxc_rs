//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1189/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1189<F: Float>(t21121: F, t854: F, t19553: F, t858: F, t884: F, t886: F, t4408: F, t6638: F, t20597: F, t2118: F, t20085: F, t21064: F, t21068: F, t21106: F, t21115: F, t21118: F, t2272: F, t2305: F, t2312: F, t6207: F, t6276: F, t6637: F, t824: F, t902: F, t905: F) -> (F, F, F) {
    let t21122 = t854 * t21121;
    let t21123 = F::cast_from(455.0_f64) / F::cast_from(324.0_f64) * t21122;
    let t21127 = t884 * t886 * t858 * t19553 / F::cast_from(48.0_f64);
    let t21128 = t4408 * t6638;
    let t21132 = t2118 * t20597;
    let t21139 = -t21064 + t21068 + t902 * t905 * t2305 * t20085 / F::cast_from(256.0_f64) + t902 * t905 * t21106 * t824 / F::cast_from(1536.0_f64) + t21115 - F::cast_from(595.0_f64) / F::cast_from(1296.0_f64) * t21118 - t21123 - t21127 + t6637 * t6276 * t21128 / F::cast_from(96.0_f64) + t6637 * t6276 * t21132 / F::cast_from(128.0_f64) - t2312 * t6207 * t2272 / F::cast_from(64.0_f64);
    (t21123, t21127, t21139)
}
