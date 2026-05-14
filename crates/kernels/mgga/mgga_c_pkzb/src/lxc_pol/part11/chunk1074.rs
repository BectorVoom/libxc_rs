//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1074/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1074<F: Float>(t12: F, t10627: F, t16425: F, t600: F, t10670: F, t1769: F, t1064: F, t10760: F, t10764: F, t1430: F, t207: F, t2732: F, t2735: F, t28874: F, t28877: F, t28885: F, t3510: F, t439: F, t8729: F, zeta_threshold: F) -> (F, F, F) {
    let t84 = t12 <= zeta_threshold;
    let t29024 = t10627 * t16425 * t600;
    let t29032 = t1769 * t10670;
    let t29049 = piecewise3(t84, 0.0, -56.0 / 81.0 * t10760 * t439 + 16.0 / 9.0 * t3510 * t1430 + 8.0 / 9.0 * t2732 * t28874 - 4.0 / 3.0 * t2735 * t28877 - 2.0 / 3.0 * t1064 * t8729 - 2.0 / 9.0 * t10764 * t439 + 2.0 / 3.0 * t207 * t28885);
    (t29024, t29032, t29049)
}
