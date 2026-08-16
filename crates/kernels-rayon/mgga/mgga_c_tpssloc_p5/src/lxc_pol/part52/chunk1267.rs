//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1267/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1267(t25365: f64, t25373: f64, t1408: f64, t6665: f64, t1530: f64, t16596: f64, t113111: f64, t113135: f64, t118376: f64, t118377: f64, t118381: f64, t118387: f64, t118393: f64, t118399: f64, t118406: f64, t1877: f64, t23290: f64, t25015: f64, t25028: f64, t2522: f64, t25372: f64, t25377: f64, t25381: f64, t25385: f64, t30753: f64, t30757: f64, t30770: f64, t32899: f64, t6670: f64, t6671: f64, t7475: f64, t7545: f64, t8370: f64) -> (f64, f64) {
    let t118407 = t25373 * t25365;
    let t118410 = t1408 * t6665;
    let t118413 = t1530 * t6665;
    let t118414 = t25373 * t118413;
    let t118417 = t25373 * t16596;
    let t118429 = -t1877 * t23290 * t32899 - 3.0_f64 * t118376 * t118377 + 3.0_f64 * t118381 * t25015 + t1877 * t30753 * t1408 / 2.0_f64 - t1877 * t6670 * t118387 - 3.0_f64 / 2.0_f64 * t2522 * t8370 * t25385 - t1877 * t6670 * t118393 - t1877 * t30757 * t25377 / 2.0_f64 - t1877 * t118399 * t6671 / 2.0_f64 + t1877 * t30770 * t25381 - t118406 + 3.0_f64 * t113135 * t118407 - t1877 * t6670 * t118410 + 2.0_f64 * t25372 * t118414 + 3.0_f64 * t113135 * t118417 + 3.0_f64 / 2.0_f64 * t2522 * t30753 * t7475 - t1877 * t113111 * t7545 / 2.0_f64 - 3.0_f64 / 2.0_f64 * t2522 * t8370 * t25028;
    (t118413, t118429)
}
