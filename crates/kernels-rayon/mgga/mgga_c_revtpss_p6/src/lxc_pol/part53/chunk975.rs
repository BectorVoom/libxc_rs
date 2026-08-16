//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 975/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk975(t33: f64, t1469: f64, t2159: f64, t27821: f64, t29329: f64, t4186: f64, t57: f64, t606: f64, t7677: f64, t8227: f64, t29005: f64, t118: f64, t1502: f64, t2163: f64, t27116: f64, t27118: f64, t27120: f64, t27122: f64, t27125: f64, t27128: f64, t27130: f64, t27132: f64, t27134: f64, t4246: f64, t4293: f64, t4297: f64, t7586: f64, t7683: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> (f64, f64) {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t29336 = piecewise3(t400, t27821, -t7677 * t1469 / 2.0_f64 - t2159 * t4186 / 2.0_f64 + t29329 * t57 / 2.0_f64 - t8227 * t606 / 2.0_f64);
    let t29337 = t29005 + t29336;
    let t29343 = -t118 * t29337 - t1502 * t7683 - t2163 * t4246 - 2.0_f64 * t4293 * t7586 - 2.0_f64 * t4297 * t7586 - t27116 - t27118 - t27120 - t27122 - t27125 - t27128 - t27130 - t27132 - t27134;
    (t29337, t29343)
}
