//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1441/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1441(t1: f64, t12612: f64, t146: f64, t1506: f64, t15274: f64, t155: f64, t19: f64, t27670: f64, t27781: f64, t3186: f64, t44090: f64, t450: f64, t451: f64, t455: f64, t458: f64, t459: f64, t45954: f64, t45968: f64, t46007: f64, t46039: f64, t464: f64, t5356: f64, t55550: f64, t55561: f64, t55598: f64, t55605: f64, t58547: f64, t58661: f64, t59023: f64, t60009: f64) -> f64 {
    let t60135 = -0.3863627328795003491e-1_f64 * t45954 - 0.51515031050600046546e-1_f64 * t45968 + 0.22477725215078486977e2_f64 * t146 * t455 * t58547 * t459 + 0.15599358861923136642e2_f64 * t155 * t464 * t58661 * t451 - 0.17581974682482873924e4_f64 * t12612 * t44090 * t1506 * t15274 - t27670 - 0.52888765211949381121e1_f64 * t55550 - 0.34034964789650479946e0_f64 * t46007 + 0.18014732272771396904e7_f64 * t27781 * t458 * t59023 * t19 + 0.12388982497197637389e3_f64 * t55561 + 0.15802725909364645561e4_f64 * t46039 * t5356 - 0.58606582274942913081e3_f64 * t55598 + 0.69688026546736710315e2_f64 * t3186 * t450 * t60009 * t1 + 0.12020173911806677527e0_f64 * t55605;
    t60135
}
