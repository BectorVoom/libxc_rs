//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 761/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk761(t589: f64, t597: f64, t187: f64, t190: f64, t5044: f64, t1860: f64, t401: f64, t1251: f64, t607: f64, t1863: f64, t1857: f64, t177: f64, t572: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5219 = t589 * t597;
    let t5241 = 0.29629629629629629629e-1_f64 * t190 * t5044 * t187;
    let t5248 = t401 * t1860;
    let t5256 = t1251 * t607;
    let t5258 = t401 * t1863;
    let t5260 = t401 * t1857;
    let t5263 = 1.0_f64 / t177 / t572;
    (t5219, t5241, t5248, t5256, t5258, t5260, t5263)
}
