//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1022/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1022<F: Float>(t10414: F, t666: F, t88252: F, t89: F, t2670: F, t88239: F, t19289: F, t193: F, t5299: F, t55558: F, t55562: F, t83652: F, t83655: F, t83683: F, t89820: F, t89824: F, t89828: F, t89834: F, t89837: F, t89840: F, t89845: F) -> (F, F, F, F) {
    let t89851 = t89 * t666 * t10414 * t88252;
    let t89855 = t89 * t666 * t2670 * t88239;
    let t89859 = t89 * t193 * t19289 * t5299;
    let t89861 = 9.0 / 4.0 * t89820 - 4.0 * t89824 + 4.0 / 3.0 * t89828 - 8.0 * t83652 + 8.0 / 3.0 * t83655 + 2.0 * t89834 + 8.0 * t89837 - 8.0 / 9.0 * t89840 - 16.0 / 9.0 * t83683 - 80.0 / 81.0 * t89845 + 112.0 / 27.0 * t55558 + 112.0 / 81.0 * t55562 + 8.0 * t89851 + 2.0 * t89855 - 36.0 * t89859;
    (t89851, t89855, t89859, t89861)
}
