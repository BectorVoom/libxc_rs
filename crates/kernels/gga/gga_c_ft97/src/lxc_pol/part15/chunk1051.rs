//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1051/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1051<F: Float>(t70141: F, t83606: F, t83619: F, t89772: F, t89775: F, t89778: F, t89781: F, t89785: F, t89789: F, t89794: F, t89798: F, t89802: F, t89807: F, t89811: F, t89815: F, t55558: F, t55562: F, t83652: F, t83655: F, t83683: F, t89820: F, t89824: F, t89828: F, t89834: F, t89837: F, t89840: F, t89845: F, t89851: F, t89855: F, t89859: F) -> (F, F) {
    let t91032 = 4.0 / 9.0 * t89772 + 8.0 / 9.0 * t89775 - t89778 / 3.0 + 8.0 / 3.0 * t89781 + 8.0 / 3.0 * t89785 + 8.0 / 3.0 * t89789 - 8.0 / 9.0 * t70141 - t89794 / 9.0 - 8.0 / 3.0 * t89798 - 2.0 / 9.0 * t89802 + 4.0 / 27.0 * t83606 - 8.0 / 3.0 * t89807 + 8.0 / 9.0 * t89811 - 8.0 / 3.0 * t89815 + 8.0 / 9.0 * t83619;
    let t91048 = 3.0 / 4.0 * t89820 - 4.0 / 3.0 * t89824 + 4.0 / 9.0 * t89828 - 8.0 / 3.0 * t83652 + 8.0 / 9.0 * t83655 + 2.0 / 3.0 * t89834 + 8.0 / 3.0 * t89837 - 8.0 / 27.0 * t89840 - 16.0 / 27.0 * t83683 - 80.0 / 243.0 * t89845 + 112.0 / 81.0 * t55558 + 112.0 / 243.0 * t55562 + 8.0 / 3.0 * t89851 + 2.0 / 3.0 * t89855 - 12.0 * t89859;
    (t91032, t91048)
}
