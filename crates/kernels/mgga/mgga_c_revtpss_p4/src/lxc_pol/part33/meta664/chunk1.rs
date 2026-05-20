//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2164/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2164<F: Float>(t108175: F, t108178: F, t108188: F, t108206: F, t25921: F, t25930: F, t25931: F, t27837: F, t27868: F, t27980: F, t28003: F, t30032: F, t30096: F, t543: F, t5658: F, t7295: F, t7301: F, t75047: F, t75051: F, t75305: F, t7910: F, t7926: F, t94602: F, t97764: F, t97785: F, t98050: F) -> F {
    let t108213 = F::cast_from(0.9757440539382783019e-2_f64) * t108175 - F::cast_from(0.17347256376410398924e1_f64) * t25930 * t25931 * t108178 + F::cast_from(0.8673628188205199462e0_f64) * t98050 * t7926 - t97785 + F::cast_from(0.8673628188205199462e0_f64) * t25921 * t30032 - F::cast_from(0.28912093960683998207e-1_f64) * t108188 + t94602 + F::cast_from(0.17347256376410398924e1_f64) * t27837 * t28003 + F::cast_from(0.8673628188205199462e0_f64) * t7295 * t7301 * t7910 * t5658 * t543 + F::cast_from(0.4336814094102599731e0_f64) * t25921 * t30096 + F::cast_from(0.26020884564615598386e1_f64) * t27868 * t97764 * t75047 - F::cast_from(0.26020884564615598386e1_f64) * t27868 * t27980 * t75051 - F::cast_from(0.8673628188205199462e0_f64) * t25930 * t25931 * t108206 + F::cast_from(0.4336814094102599731e0_f64) * t27868 * t25931 * t75305;
    t108213
}
