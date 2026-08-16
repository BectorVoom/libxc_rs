//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2664/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2664(t1268: f64, t12725: f64, t12734: f64, t12739: f64, t12813: f64, t1458: f64, t19451: f64, t19456: f64, t19534: f64, t2314: f64, t2363: f64, t26114: f64, t4028: f64, t4072: f64, t45632: f64, t5113: f64, t5493: f64, t55410: f64, t55568: f64, t55927: f64, t55934: f64, t55943: f64, t55946: f64, t55962: f64, t55967: f64, t671: f64, t7676: f64, t88: f64, t9348: f64) -> f64 {
    let t55969 = 2.0_f64 * t1268 * t55568 + 8.0_f64 * t12725 * t4072 + 4.0_f64 * t12734 * t5493 + 2.0_f64 * t12739 * t5493 + 4.0_f64 * t12813 * t4028 + 4.0_f64 * t12813 * t7676 + 4.0_f64 * t1458 * t45632 + 8.0_f64 * t1458 * t55934 + 4.0_f64 * t1458 * t55962 + 2.0_f64 * t19451 * t2363 + 8.0_f64 * t19456 * t4072 + 4.0_f64 * t19534 * t2314 + 4.0_f64 * t19534 * t5113 + 8.0_f64 * t26114 * t4072 + 2.0_f64 * t5493 * t9348 + 4.0_f64 * t55410 * t88 + 4.0_f64 * t55943 * t671 + t55927 + 2.0_f64 * t55946 + 2.0_f64 * t55967;
    t55969
}
