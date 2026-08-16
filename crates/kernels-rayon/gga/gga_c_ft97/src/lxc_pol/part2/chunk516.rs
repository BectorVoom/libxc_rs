//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 516/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk516(t3099: f64, t409: f64, t64: f64, t1603: f64, t1617: f64, t1624: f64, t1669: f64, t3019: f64, t3022: f64, t3025: f64, t3030: f64, t3034: f64, t3038: f64, t3058: f64, t3061: f64, t3067: f64, t3071: f64, t3076: f64, t3078: f64, t372: f64, t374: f64, t399: f64, t940: f64) -> (f64, f64) {
    let t3100 = t409 * t3099;
    let t3101 = t64 * t3100;
    let t3102 = 0.67598802253579164263e-4_f64 * t3019 * t3022 - 0.23254900946437792e-1_f64 * t1603 * t374 * t3025 - 0.68920324918704953981e-4_f64 * t1617 * t3030 + 0.11627450473218896e-1_f64 * t1624 * t3034 + 0.23254900946437792e-2_f64 * t372 * t3038 - 0.11627450473218896e-1_f64 * t372 * t3058 + 0.19365723406274399941e-3_f64 * t372 * t3061 + 0.11627450473218896e-1_f64 * t1624 * t3067 - 2.0_f64 * t1669 * t3071 - 0.59273806478425129876e-2_f64 * t940 * t399 + 2.0_f64 * t3076 * t3078 - t3101;
    (t3101, t3102)
}
