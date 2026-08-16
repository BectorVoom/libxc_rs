//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1195/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1195(t33: f64, t265: f64, t502: f64, t127181: f64, t132085: f64, t127288: f64, t1469: f64, t33544: f64, t35008: f64, t4186: f64, t57: f64, t606: f64, t8960: f64, t118: f64, t127340: f64, t129308: f64, t129436: f64, t129437: f64, t129438: f64, t129440: f64, t129445: f64, t129447: f64, t129449: f64, t129452: f64, t129455: f64, t129457: f64, t129459: f64, t129461: f64, t129463: f64, t129465: f64, t131384: f64, t131387: f64, t29459: f64, t569: f64, t7586: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> f64 {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t132086 = piecewise3(t503, t132085, t127181);
    let t132093 = piecewise3(t400, t127288, t132086 * t57 / 2.0_f64 - t33544 * t1469 / 2.0_f64 - t35008 * t606 / 2.0_f64 - t8960 * t4186 / 2.0_f64);
    let t132107 = -4.0_f64 * t7586 * t29459 - 2.0_f64 * t129436 - 2.0_f64 * t129437 + 6.0_f64 * t129438 + (t131384 + t131387) * t569 - t118 * (t129308 + t132093) + 6.0_f64 * t129440 - 4.0_f64 * t129445 - 4.0_f64 * t129447 - 4.0_f64 * t129449 - 4.0_f64 * t129452 - 2.0_f64 * t129455 - t127340 - 4.0_f64 * t129457 - 4.0_f64 * t129459 - 4.0_f64 * t129461 - 4.0_f64 * t129463 - 4.0_f64 * t129465;
    t132107
}
