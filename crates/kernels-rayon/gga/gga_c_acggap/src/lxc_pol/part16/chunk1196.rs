//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1196/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1196(t1165: f64, t34248: f64, t5532: f64, t7564: f64, t5537: f64, t8600: f64, t30219: f64, t9670: f64, t36274: f64, t36284: f64, t36287: f64, t36293: f64, t36294: f64, t36300: f64, t36303: f64, t37940: f64, t40465: f64, t40467: f64, t40469: f64, t40472: f64, t40474: f64, t40477: f64) -> f64 {
    let t40481 = t7564 * t1165 * t34248 * t5532;
    let t40485 = t7564 * t1165 * t8600 * t5537;
    let t40487 = t30219 * t9670;
    let t40489 = -0.12579236915841660827e-2_f64 * t40465 + 0.17149607247227894789e-2_f64 * t40467 + 0.17149607247227894789e-2_f64 * t40469 + t36274 + t36284 - t36287 - t37940 + t36293 - 0.27953859812981468504e-2_f64 * t36294 + t36300 + t36303 + 0.17149607247227894789e-2_f64 * t40472 + 0.42874018118069736972e-3_f64 * t40474 + t40477 / 16.0_f64 + 0.94344276868812456205e-2_f64 * t40481 - 0.37737710747524982482e-2_f64 * t40485 + 0.21437009059034868486e-2_f64 * t40487;
    t40489
}
