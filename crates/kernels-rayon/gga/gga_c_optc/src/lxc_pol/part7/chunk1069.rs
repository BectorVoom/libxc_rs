//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1069/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1069(t7122: f64, t7125: f64, t7116: f64, t7113: f64, t6933: f64, t7110: f64, t2004: f64, t2123: f64, t2127: f64, t6986: f64, t9917: f64, t151: f64, t2124: f64, t2126: f64, t2168: f64, t22160: f64, t22169: f64, t22202: f64, t22211: f64, t22858: f64, t22864: f64, t22871: f64, t22880: f64, t3467: f64) -> f64 {
    let t23203 = t7122 * t7125;
    let t23205 = t7122 * t7116;
    let t23213 = t7122 * t7113;
    let t23215 = t7110 * t6933;
    let t23219 = t2123 * t2004;
    let t23220 = t23219 * t2127;
    let t23228 = t9917 * t6986;
    let t23233 = -0.34772645959155031419e0_f64 * t2124 * t151 * t22202 + 0.24182738140014814697e0_f64 * t2168 * t22880 - 0.48681704342817043985e1_f64 * t23203 + 0.24340852171408521992e1_f64 * t23205 + 0.69545291918310062836e0_f64 * t2124 * t2126 * t22160 - 0.52158968938732547128e0_f64 * t2124 * t151 * t22858 - 0.48681704342817043985e1_f64 * t23213 + 0.16927916698010370288e2_f64 * t23215 + 0.72548214420044444093e1_f64 * t2168 * t22169 + 0.16227234780939014662e2_f64 * t23220 + 0.69545291918310062836e0_f64 * t2124 * t2126 * t22871 - 0.2086358757549301885e1_f64 * t3467 * t2126 * t22864 + 0.9736340868563408797e1_f64 * t23228 + 0.69545291918310062836e0_f64 * t3467 * t151 * t22211;
    t23233
}
