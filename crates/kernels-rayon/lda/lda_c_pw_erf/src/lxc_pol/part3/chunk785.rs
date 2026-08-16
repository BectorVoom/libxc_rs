//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 785/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk785(t4610: f64, t5250: f64, t519: f64, t4218: f64, t4220: f64, t4225: f64, t4227: f64, t4235: f64, t5213: f64, t5217: f64, t5224: f64, t5228: f64, t5233: f64, t5236: f64, t5240: f64, t5242: f64, t5246: f64, t5249: f64) -> (f64, f64, f64) {
    let t5251 = t5250 * t4610;
    let t5253 = 32.0_f64 / 81.0_f64 * t519 * t5251;
    let t5254 = 4.0_f64 / 3.0_f64 * t4218 + 16.0_f64 / 3.0_f64 * t4220 + t5213 + t5217 + 4.0_f64 / 3.0_f64 * t4225 + 8.0_f64 / 3.0_f64 * t4227 + t4235 - t5224 + t5228 + t5233 - t5236 + t5240 - t5242 - t5246 + t5249 + t5253;
    (t5251, t5253, t5254)
}
