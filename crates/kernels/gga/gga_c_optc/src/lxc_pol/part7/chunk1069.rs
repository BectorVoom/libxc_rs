//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1069/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1069<F: Float>(t7122: F, t7125: F, t7116: F, t7113: F, t6933: F, t7110: F, t2004: F, t2123: F, t2127: F, t6986: F, t9917: F, t151: F, t2124: F, t2126: F, t2168: F, t22160: F, t22169: F, t22202: F, t22211: F, t22858: F, t22864: F, t22871: F, t22880: F, t3467: F) -> F {
    let t23203 = t7122 * t7125;
    let t23205 = t7122 * t7116;
    let t23213 = t7122 * t7113;
    let t23215 = t7110 * t6933;
    let t23219 = t2123 * t2004;
    let t23220 = t23219 * t2127;
    let t23228 = t9917 * t6986;
    let t23233 = -F::new(0.34772645959155031419e0) * t2124 * t151 * t22202 + F::new(0.24182738140014814697e0) * t2168 * t22880 - F::new(0.48681704342817043985e1) * t23203 + F::new(0.24340852171408521992e1) * t23205 + F::new(0.69545291918310062836e0) * t2124 * t2126 * t22160 - F::new(0.52158968938732547128e0) * t2124 * t151 * t22858 - F::new(0.48681704342817043985e1) * t23213 + F::new(0.16927916698010370288e2) * t23215 + F::new(0.72548214420044444093e1) * t2168 * t22169 + F::new(0.16227234780939014662e2) * t23220 + F::new(0.69545291918310062836e0) * t2124 * t2126 * t22871 - F::new(0.2086358757549301885e1) * t3467 * t2126 * t22864 + F::new(0.9736340868563408797e1) * t23228 + F::new(0.69545291918310062836e0) * t3467 * t151 * t22211;
    t23233
}
