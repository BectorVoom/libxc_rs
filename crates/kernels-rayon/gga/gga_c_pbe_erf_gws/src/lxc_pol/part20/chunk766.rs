//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 766/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk766(t1641: f64, t50: f64, t188: f64, t9: f64, t191: f64, t784: f64, t190: f64, t212: f64, t1251: f64, t658: f64, t205: f64, t626: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5002 = 1.0_f64 / t1641 / t50;
    let t5018 = t9 * t188;
    let t5044 = t784 * t191;
    let t5047 = 0.29629629629629629629e-1_f64 * t190 * t5044 * t212;
    let t5052 = t1251 * t658;
    let t5060 = 1.0_f64 / t205 / t626;
    (t5002, t5018, t5044, t5047, t5052, t5060)
}
