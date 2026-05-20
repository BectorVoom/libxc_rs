//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3122/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3122<F: Float>(t300: F, t81781: F, t81796: F, t81835: F, t81877: F, t82006: F, t82045: F, t82049: F, t82115: F, t24864: F, t460: F, t5219: F, t6695: F) -> (F, F, F) {
    let t82119 = t300 * (t81781 + t81796 + t81835 + t81877 + t82006 + t82045 + t82049 + t82115);
    let t82147 = t460 * t24864;
    let t82150 = t5219 * t6695;
    (t82119, t82147, t82150)
}
