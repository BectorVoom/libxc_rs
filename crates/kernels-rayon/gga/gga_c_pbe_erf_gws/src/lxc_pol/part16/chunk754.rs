//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 754/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk754(t1887: f64, t5018: f64, t1820: f64, t1718: f64, t401: f64, t1699: f64, t395: f64, t191: f64, t784: f64, t190: f64, t212: f64, t1251: f64, t658: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5019 = t5018 * t1887;
    let t5020 = t1820 * t5019;
    let t5022 = t401 * t1718;
    let t5042 = t395 * t1699;
    let t5044 = t784 * t191;
    let t5047 = 0.29629629629629629629e-1_f64 * t190 * t5044 * t212;
    let t5052 = t1251 * t658;
    (t5020, t5022, t5042, t5044, t5047, t5052)
}
