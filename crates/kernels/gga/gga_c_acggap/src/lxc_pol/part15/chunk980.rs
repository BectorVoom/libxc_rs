//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 980/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk980<F: Float>(t1839: F, t1992: F, t7585: F, t7842: F, t1089: F, t1894: F, t2079: F, t2080: F, t137: F, t5674: F, t1459: F, t598: F, t1980: F, t38892: F, t7458: F, t1967: F, t9543: F) -> (F, F, F, F, F, F) {
    let t39213 = t7585 * t7842 * t1992 * t1839;
    let t39217 = t2079 * t1089 * t1894 * t2080;
    let t39219 = t137 * t5674;
    let t39222 = t598 * t1089 * t1459 * t39219;
    let t39226 = t1980 * t7458 * t1459 * t38892;
    let t39228 = t1967 * t9543;
    (t39213, t39217, t39219, t39222, t39226, t39228)
}
