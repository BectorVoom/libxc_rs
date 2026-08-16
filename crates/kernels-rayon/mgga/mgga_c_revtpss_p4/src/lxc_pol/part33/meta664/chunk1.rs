//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2164/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2164(t108175: f64, t108178: f64, t108188: f64, t108206: f64, t25921: f64, t25930: f64, t25931: f64, t27837: f64, t27868: f64, t27980: f64, t28003: f64, t30032: f64, t30096: f64, t543: f64, t5658: f64, t7295: f64, t7301: f64, t75047: f64, t75051: f64, t75305: f64, t7910: f64, t7926: f64, t94602: f64, t97764: f64, t97785: f64, t98050: f64) -> f64 {
    let t108213 = 0.9757440539382783019e-2_f64 * t108175 - 0.17347256376410398924e1_f64 * t25930 * t25931 * t108178 + 0.8673628188205199462e0_f64 * t98050 * t7926 - t97785 + 0.8673628188205199462e0_f64 * t25921 * t30032 - 0.28912093960683998207e-1_f64 * t108188 + t94602 + 0.17347256376410398924e1_f64 * t27837 * t28003 + 0.8673628188205199462e0_f64 * t7295 * t7301 * t7910 * t5658 * t543 + 0.4336814094102599731e0_f64 * t25921 * t30096 + 0.26020884564615598386e1_f64 * t27868 * t97764 * t75047 - 0.26020884564615598386e1_f64 * t27868 * t27980 * t75051 - 0.8673628188205199462e0_f64 * t25930 * t25931 * t108206 + 0.4336814094102599731e0_f64 * t27868 * t25931 * t75305;
    t108213
}
