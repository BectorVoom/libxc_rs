//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1588/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1588<F: Float>(t1904: F, t22445: F, t689: F, t22974: F, t47603: F, t686: F, t72: F, t213: F, t22964: F, t13729: F, t2782: F, t556: F, t6918: F) -> (F, F, F, F) {
    let t86682 = t689 * t22445 * t1904;
    let t86699 = t47603 * t22974 * t72 * t686;
    let t86701 = t213 * t22964;
    let t86712 = t2782 * t556 * t13729 * t6918;
    (t86682, t86699, t86701, t86712)
}
