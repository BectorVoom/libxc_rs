//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 68/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk68(t153: f64, t185: f64, t152: f64, t157: f64, t182: f64, t68: f64) -> (f64, f64, f64, f64, f64) {
    let t186 = t153 * t185;
    let t187 = t152 * t157;
    let t189 = 0.19751673498613801407e-1_f64 * t187 * t182;
    let t190 = f64::ln(2.0_f64);
    let t191 = 1.0_f64 - t190;
    let t192 = 1.0_f64 / t68;
    let t193 = t191 * t192;
    (t186, t187, t189, t191, t193)
}
