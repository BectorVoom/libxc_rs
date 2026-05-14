//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1066/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1066<F: Float>(t12: F, t10513: F, t10518: F, t1430: F, t16232: F, t1642: F, t2540: F, t28874: F, t28877: F, t28885: F, t439: F, t6767: F, t6770: F, t87: F, t8721: F, t8729: F, t3374: F, t507: F, zeta_threshold: F) -> (F, F) {
    let t84 = t12 <= zeta_threshold;
    let t28889 = piecewise3(t84, 0.0, 40.0 / 81.0 * t16232 * t10513 * t439 - 16.0 / 9.0 * t8721 * t1430 - 8.0 / 9.0 * t6767 * t28874 + 8.0 / 3.0 * t6770 * t28877 + 4.0 / 3.0 * t2540 * t8729 + 4.0 / 9.0 * t1642 * t10518 * t439 + 4.0 / 3.0 * t87 * t28885);
    let t28895 = t3374 * t507;
    (t28889, t28895)
}
