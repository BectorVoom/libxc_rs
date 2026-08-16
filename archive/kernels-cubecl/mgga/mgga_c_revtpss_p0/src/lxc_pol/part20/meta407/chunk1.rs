//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1507/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1507<F: Float>(t11977: F, t3173: F, t12009: F, t12013: F, t11916: F, t11999: F, t3043: F, t3140: F, t3149: F, t11239: F, t989: F, t11629: F) -> (F, F, F, F, F, F, F) {
    let t42658 = t11977 * t3173;
    let t42660 = t12013 * t12009;
    let t42662 = t11999 * t11916;
    let t42664 = t3043 * t3140;
    let t42665 = t42664 * t3149;
    let t42668 = t989 * t11239;
    let t42669 = t42668 * t11629;
    (t42658, t42660, t42662, t42664, t42665, t42668, t42669)
}
