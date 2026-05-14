//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1056/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1056<F: Float>(t70142: F, t83606: F, t89772: F, t89775: F, t89781: F, t89785: F, t89789: F, t89794: F, t89798: F, t89802: F, t89807: F, t89811: F, t89815: F, t72080: F, t72082: F, t83619: F, t83652: F, t83655: F, t83683: F, t89824: F, t89828: F, t89834: F, t89837: F, t89840: F, t89845: F, t89851: F, t89855: F) -> (F, F) {
    let t91158 = 2.0 / 9.0 * t89772 + 4.0 / 9.0 * t89775 + 4.0 / 3.0 * t89781 + 4.0 / 3.0 * t89785 + 4.0 / 3.0 * t89789 - t70142 - t89794 / 18.0 - 4.0 / 3.0 * t89798 - t89802 / 9.0 + 2.0 / 27.0 * t83606 - 4.0 / 3.0 * t89807 + 4.0 / 9.0 * t89811 - 4.0 / 3.0 * t89815;
    let t91171 = 4.0 / 9.0 * t83619 - 2.0 / 3.0 * t89824 + 2.0 / 9.0 * t89828 - 4.0 / 3.0 * t83652 + 4.0 / 9.0 * t83655 + t89834 / 3.0 + 4.0 / 3.0 * t89837 - 4.0 / 27.0 * t89840 - 8.0 / 27.0 * t83683 - 40.0 / 243.0 * t89845 + t72080 + t72082 + 4.0 / 3.0 * t89851 + t89855 / 3.0;
    (t91158, t91171)
}
