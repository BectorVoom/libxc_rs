//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2922/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2922(t14344: f64, t4488: f64, t959: f64, t11094: f64, t5946: f64, t1068: f64, t3213: f64, t4700: f64, t60842: f64, t60844: f64, t60847: f64, t60850: f64, t60852: f64, t60855: f64, t60857: f64, t60860: f64, t60862: f64, t60864: f64, t60866: f64, t60867: f64) -> (f64, f64) {
    let t60873 = 0.23392894490538584828e1_f64 * t959 * t4488 * t14344;
    let t60874 = t5946 * t11094;
    let t60878 = -2.0_f64 * t1068 * t4700 * t60867 + 2.0_f64 * t3213 * t4700 * t60874 - t60842 - t60844 + t60847 - t60850 + t60852 - t60855 + t60857 - t60860 - t60862 - t60864 - t60866 + t60873;
    (t60873, t60878)
}
