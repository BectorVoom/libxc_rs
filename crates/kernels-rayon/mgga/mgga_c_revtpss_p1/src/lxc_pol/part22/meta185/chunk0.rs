//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1189/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1189(t45: f64, t57: f64, t1469: f64, t2375: f64, t4186: f64, t606: f64, t78: f64, t2382: f64, t81: f64, t162: f64, t187: f64, t150: f64, t190: f64, t1532: f64, t750: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t151 = t45 <= zeta_threshold;
    let t155 = t57 <= zeta_threshold;
    let t4377 = t2375 * t1469;
    let t4383 = piecewise3(t151, 0.0_f64, 4.0_f64 / 9.0_f64 * t4377 * t606 + 4.0_f64 / 3.0_f64 * t78 * t4186);
    let t4384 = t2382 * t1469;
    let t4390 = piecewise3(t155, 0.0_f64, 4.0_f64 / 9.0_f64 * t4384 * t606 - 4.0_f64 / 3.0_f64 * t81 * t4186);
    let t4391 = t4383 + t4390;
    let t4392 = t4391 * t162;
    let t4394 = 0.19751673498613801407e-1_f64 * t4392 * t187;
    let t4395 = t150 * t4391;
    let t4396 = t4395 * t190;
    let t4397 = t1532 * t750;
    (t4377, t4384, t4391, t4392, t4394, t4395, t4396, t4397)
}
