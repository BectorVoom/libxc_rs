//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 785/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk785<F: Float>(t12993: F, t7014: F, t10215: F, t123: F, t883: F, t2487: F, t2488: F, t10151: F, t2464: F, t2465: F, t10417: F, t1415: F, t7030: F) -> (F, F, F, F, F) {
    let t41631 = t7014 * t12993;
    let t41634 = t10215 * t123 * t883;
    let t41636 = t2487 * t2488 * t41634;
    let t41640 = t2487 * t2464 * t2465 * t10151;
    let t41643 = t1415 * t10417 * t7030;
    (t41631, t41634, t41636, t41640, t41643)
}
