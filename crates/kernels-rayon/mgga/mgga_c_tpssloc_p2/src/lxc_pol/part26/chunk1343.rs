//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1343/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1343(t11972: f64, t1266: f64, t2364: f64, t24924: f64, t24935: f64, t27888: f64, t650: f64, t7266: f64, t81469: f64, t83672: f64, t83674: f64, t83677: f64, t83679: f64, t83681: f64, t83684: f64, t83687: f64, t83692: f64, t83694: f64, t83698: f64, t83862: f64, t83866: f64, t83869: f64, t83876: f64, t83880: f64) -> f64 {
    let t85595 = -2.0_f64 * t11972 * t7266 - 6.0_f64 * t1266 * t24935 - 6.0_f64 * t2364 * t27888 - 3.0_f64 * t24924 * t650 + t81469 - t83672 - t83674 - t83677 - t83679 - t83681 - t83684 + t83687 - t83692 - t83694 - t83698 + t83862 + t83866 - t83869 + t83876 + t83880;
    t85595
}
