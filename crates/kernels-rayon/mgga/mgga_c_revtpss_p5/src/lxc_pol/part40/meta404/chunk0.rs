//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1480/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1480(t3: f64, t31204: f64, t2198: f64, t2327: f64, t116: f64, t8320: f64, t670: f64, t2371: f64, t8342: f64, t117: f64, t31157: f64, t1459: f64, t1461: f64, t2207: f64, t2209: f64, t4158: f64, t4162: f64, t4165: f64, t572: f64, t573: f64, t8336: f64, t8343: f64, t8346: f64, param_d: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t31205 = t3 * t31204;
    let t31217 = param_d * t31204;
    let t31231 = t2327 * t2198;
    let t31234 = t116 * t8320;
    let t31235 = t31234 * t670;
    let t31238 = t8342 * t2371;
    let t31241 = t117 * t31157;
    let t31244 = 12.0_f64 * t1459 * t8343 + 6.0_f64 * t1459 * t8346 + 6.0_f64 * t1461 * t8336 + 6.0_f64 * t2207 * t4162 + 3.0_f64 * t2207 * t4165 + 3.0_f64 * t2209 * t4158 + t31217 * t573 + 6.0_f64 * t31231 * t572 + 12.0_f64 * t31235 * t572 + 6.0_f64 * t31238 * t572 + 3.0_f64 * t31241 * t572;
    (t31205, t31217, t31231, t31234, t31235, t31238, t31241, t31244)
}
