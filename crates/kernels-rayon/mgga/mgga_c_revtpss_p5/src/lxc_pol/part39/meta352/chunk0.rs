//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1205/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1205(t13312: f64, t190: f64, t706: f64, t4391: f64, t705: f64, t707: f64, t189: f64, t4186: f64, t606: f64, t4401: f64, t10579: f64, t2411: f64, t4537: f64) -> (f64, f64, f64, f64, f64) {
    let t14383 = t190 * t13312;
    let t14385 = 4.0_f64 * t706 * t14383;
    let t14386 = t705 * t4391;
    let t14388 = 8.0_f64 * t14386 * t707;
    let t14389 = t189 * t4186;
    let t14390 = t14389 * t606;
    let t14392 = 24.0_f64 * t4401 * t14390;
    let t14396 = 0.21687162600603479684e-1_f64 * t10579;
    let t14397 = t4537 * t2411;
    (t14385, t14388, t14392, t14396, t14397)
}
