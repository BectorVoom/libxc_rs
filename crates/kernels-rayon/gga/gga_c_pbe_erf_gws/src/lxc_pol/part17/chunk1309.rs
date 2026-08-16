//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1309/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1309(t4028: f64, t9135: f64, t14015: f64, t9655: f64, t51421: f64, t9490: f64, t14011: f64, t9588: f64, t14498: f64, t9353: f64, t51256: f64, t54158: f64, t54160: f64, t54162: f64, t54164: f64, t54167: f64, t54168: f64) -> f64 {
    let t54170 = t4028 * t9135;
    let t54173 = t14015 * t9655;
    let t54175 = t51421 * t9490;
    let t54177 = t14011 * t9588;
    let t54179 = t14498 * t9353;
    let t54181 = -t54158 / 48.0_f64 - t54160 / 24.0_f64 - t54162 / 192.0_f64 + t54164 / 96.0_f64 + t54167 + t54168 / 24.0_f64 + t54170 / 48.0_f64 + 7.0_f64 / 144.0_f64 * t51256 - t54173 / 96.0_f64 + 5.0_f64 / 192.0_f64 * t54175 + t54177 / 96.0_f64 - t54179 / 64.0_f64;
    t54181
}
