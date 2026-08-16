//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1343/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1343<F: Float>(t11972: F, t1266: F, t2364: F, t24924: F, t24935: F, t27888: F, t650: F, t7266: F, t81469: F, t83672: F, t83674: F, t83677: F, t83679: F, t83681: F, t83684: F, t83687: F, t83692: F, t83694: F, t83698: F, t83862: F, t83866: F, t83869: F, t83876: F, t83880: F) -> F {
    let t85595 = -F::cast_from(2.0_f64) * t11972 * t7266 - F::cast_from(6.0_f64) * t1266 * t24935 - F::cast_from(6.0_f64) * t2364 * t27888 - F::cast_from(3.0_f64) * t24924 * t650 + t81469 - t83672 - t83674 - t83677 - t83679 - t83681 - t83684 + t83687 - t83692 - t83694 - t83698 + t83862 + t83866 - t83869 + t83876 + t83880;
    t85595
}
