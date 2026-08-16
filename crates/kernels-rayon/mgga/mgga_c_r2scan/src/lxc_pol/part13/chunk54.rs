//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 54/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk54(t44: f64, t51: f64, t132: f64, t133: f64, t129: f64, t130: f64, t98: f64, t99: f64, t101: f64, t108: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t45 = t44 <= zeta_threshold;
    let t52 = t51 <= zeta_threshold;
    let t134 = t132 * t133;
    let t135 = t129 * t130 * t134;
    let t138 = t98 * zeta_threshold;
    let t139 = t99 * t44;
    let t140 = piecewise3(t45, t138, t139);
    let t141 = t101 * t51;
    let t142 = piecewise3(t52, t138, t141);
    let t144 = t140 / 2.0_f64 + t142 / 2.0_f64;
    let t146 = t108 / t144;
    (t134, t135, t139, t141, t144, t146)
}
