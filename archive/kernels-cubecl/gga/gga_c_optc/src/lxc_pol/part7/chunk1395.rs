//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1395/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1395<F: Float>(t27333: F, t4464: F, t8969: F, t25560: F, t4463: F, t9115: F, t1162: F, t1179: F, t123: F, t12567: F, t12568: F, t12612: F, t19: F, t27204: F, t27210: F, t27215: F, t27346: F, t27383: F, t27385: F, t27449: F, t27557: F, t27671: F, t27755: F, t27758: F, t27761: F, t27768: F, t27771: F, t27778: F, t27781: F, t27786: F, t27792: F, t27795: F, t27798: F, t3120: F, t438: F, t450: F, t458: F, t8516: F, t8537: F, t8915: F, t894: F, t9058: F, t914: F, t935: F) -> F {
    let t27801 = t4464 * t27333 * t8969;
    let t27803 = t4463 * t25560;
    let t27815 = t9115 * t25560;
    let t27824 = -F::cast_from(0.23181763972770020945e0_f64) * t27755 + F::cast_from(0.15146801702008125515e1_f64) * t27758 + F::cast_from(0.15454509315180013964e0_f64) * t27761 + F::cast_from(0.10431793787746509425e1_f64) * t1162 * t914 * t8537 * t27346 - F::cast_from(0.15146801702008125515e1_f64) * t27768 - F::cast_from(0.11721316454988582616e4_f64) * t27771 - F::cast_from(0.38640729216933594422e6_f64) * t27215 * t450 * t27671 * t438 - F::cast_from(0.30972456242994093474e2_f64) * t27778 + F::cast_from(0.18014732272771396904e7_f64) * t27781 * t458 * t27204 * t19 - F::cast_from(0.27022098409157095356e7_f64) * t27786 * t458 * t27210 * t19 + F::cast_from(0.10324152080998031158e2_f64) * t27792 + F::cast_from(0.69310201356862480534e1_f64) * t27795 + F::cast_from(0.3118959061058811624e2_f64) * t27798 - F::cast_from(0.58606582274942913081e3_f64) * t27801 + F::cast_from(0.75587607063262836759e5_f64) * t27803 * t27383 * t935 * t3120 - F::cast_from(0.17581974682482873924e4_f64) * t12612 * t27557 * t123 * t9058 + F::cast_from(0.6058720680803250206e1_f64) * t12567 * t12568 * t27449 + F::cast_from(0.45352564237957702055e6_f64) * t27815 * t27383 * t8915 * t27385 - F::cast_from(0.30228422675018518374e0_f64) * t1179 * t894 * t8516 * t27346;
    t27824
}
