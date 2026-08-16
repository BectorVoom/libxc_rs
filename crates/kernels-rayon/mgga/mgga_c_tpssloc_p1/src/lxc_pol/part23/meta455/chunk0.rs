//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1313/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1313(t59028: f64, t145: f64, t185: f64, t75929: f64, t39658: f64, t41258: f64, t41262: f64, t76024: f64, t76025: f64, t76026: f64, t76027: f64, t76030: f64, t76031: f64, t76034: f64) -> (f64, f64, f64) {
    let t76035 = 0.10389515463408878255e3_f64 * t59028;
    let t76037 = t145 * t75929 * t185;
    let t76038 = t76024 + t76025 - t41258 - t41262 - t76026 + t76027 + t76030 - t39658 + t76031 + t76034 - t76035 + t76037;
    (t76035, t76037, t76038)
}
