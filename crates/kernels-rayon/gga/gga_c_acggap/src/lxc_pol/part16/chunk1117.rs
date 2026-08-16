//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1117/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1117(t142: f64, t6379: f64, t8806: f64, t6383: f64, t1318: f64, t507: f64, t7436: f64, t6388: f64, t5906: f64, t30725: f64, t30729: f64, t34746: f64, t34754: f64, t37233: f64, t39438: f64, t39442: f64, t39447: f64, t39451: f64, t39454: f64, t39458: f64, t39462: f64) -> f64 {
    let t39465 = t8806 * t142 * t6379;
    let t39468 = t8806 * t142 * t6383;
    let t39471 = t7436 * t507 * t1318;
    let t39474 = t8806 * t142 * t6388;
    let t39477 = t7436 * t142 * t5906;
    let t39479 = -t34746 + 0.52413487149340253447e-3_f64 * t39438 + t37233 + 0.31448092289604152068e-3_f64 * t39442 + t34754 + 0.15724046144802076034e-2_f64 * t30725 + t30729 - 0.15724046144802076034e-2_f64 * t39447 + 0.28582678745379824648e-3_f64 * t39451 + 0.42874018118069736972e-3_f64 * t39454 + 0.62896184579208304136e-3_f64 * t39458 + 0.62896184579208304136e-3_f64 * t39462 - t39465 / 16.0_f64 + t39468 / 8.0_f64 + t39471 / 24.0_f64 + t39474 / 16.0_f64 + t39477 / 48.0_f64;
    t39479
}
