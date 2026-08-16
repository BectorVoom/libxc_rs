//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1371/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1371(t14682: f64, t3140: f64, t3989: f64, t57321: f64, t13815: f64, t3781: f64, t833: f64, t850: f64, t11737: f64, t1193: f64, t14802: f64, t2408: f64, t2409: f64, t26654: f64, t3060: f64, t3207: f64, t3212: f64, t35566: f64, t4155: f64, t53614: f64, t54492: f64, t54505: f64, t54532: f64, t57542: f64, t57545: f64, t57551: f64, t57555: f64, t57570: f64, t9283: f64) -> f64 {
    let t57574 = t3989 * t14682 * t57321 * t3140;
    let t57578 = t850 * t3781 * t13815 * t833;
    let t57580 = t54492 + t2408 * t2409 * t26654 * t4155 / 24.0_f64 + 7.0_f64 / 144.0_f64 * t57542 + t54505 - t57545 / 48.0_f64 - t3207 * t9283 * t1193 * t11737 / 16.0_f64 + t57551 / 48.0_f64 + t57555 / 1536.0_f64 - t2408 * t35566 * t14802 / 12.0_f64 + t54532 - t2408 * t9283 * t53614 * t3212 / 12.0_f64 - t2408 * t9283 * t53614 * t3060 / 12.0_f64 - t57570 / 512.0_f64 + t57574 / 1536.0_f64 + t57578 / 96.0_f64;
    t57580
}
