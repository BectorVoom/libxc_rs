//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1441/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1441<F: Float>(t1: F, t12612: F, t146: F, t1506: F, t15274: F, t155: F, t19: F, t27670: F, t27781: F, t3186: F, t44090: F, t450: F, t451: F, t455: F, t458: F, t459: F, t45954: F, t45968: F, t46007: F, t46039: F, t464: F, t5356: F, t55550: F, t55561: F, t55598: F, t55605: F, t58547: F, t58661: F, t59023: F, t60009: F) -> F {
    let t60135 = -F::cast_from(0.3863627328795003491e-1_f64) * t45954 - F::cast_from(0.51515031050600046546e-1_f64) * t45968 + F::cast_from(0.22477725215078486977e2_f64) * t146 * t455 * t58547 * t459 + F::cast_from(0.15599358861923136642e2_f64) * t155 * t464 * t58661 * t451 - F::cast_from(0.17581974682482873924e4_f64) * t12612 * t44090 * t1506 * t15274 - t27670 - F::cast_from(0.52888765211949381121e1_f64) * t55550 - F::cast_from(0.34034964789650479946e0_f64) * t46007 + F::cast_from(0.18014732272771396904e7_f64) * t27781 * t458 * t59023 * t19 + F::cast_from(0.12388982497197637389e3_f64) * t55561 + F::cast_from(0.15802725909364645561e4_f64) * t46039 * t5356 - F::cast_from(0.58606582274942913081e3_f64) * t55598 + F::cast_from(0.69688026546736710315e2_f64) * t3186 * t450 * t60009 * t1 + F::cast_from(0.12020173911806677527e0_f64) * t55605;
    t60135
}
