//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3174/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3174<F: Float>(t12469: F, t1737: F, t3362: F, t462: F, t2439: F, t5101: F, t16870: F, t698: F, t1729: F, t9303: F, t16894: F, t16897: F) -> (F, F, F, F, F, F, F) {
    let t58005 = t1737 * t12469;
    let t58027 = t462 * t3362;
    let t58145 = t2439 * t5101;
    let t58147 = t698 * t16870;
    let t58153 = t9303 * t1729;
    let t58158 = t698 * t16894;
    let t58160 = t698 * t16897;
    (t58005, t58027, t58145, t58147, t58153, t58158, t58160)
}
