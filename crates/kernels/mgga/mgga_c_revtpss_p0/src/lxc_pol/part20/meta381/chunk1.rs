//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1384/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1384<F: Float>(t40781: F, t857: F, t10722: F, t2479: F, t14832: F, t2430: F, t2475: F, t2661: F, t775: F, t2699: F, t2729: F, t2732: F) -> (F, F, F, F) {
    let t40782 = t40781 * t857;
    let t40784 = t10722 * t2479;
    let t40789 = t2661 * t14832 * t2475 * t775 * t2430;
    let t40791 = t2699 * t2729;
    let t40792 = t40791 * t2732;
    (t40782, t40784, t40789, t40792)
}
