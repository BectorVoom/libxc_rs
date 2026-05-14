//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 229/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk229<F: Float>(t15: F, t787: F, t130: F, t2: F, t4: F, t88: F, t128: F, t97: F, t728: F, t11: F, t122: F, t144: F, t145: F, t148: F, t745: F, t784: F, t85: F) -> (F, F, F, F, F, F) {
    let t788 = t787 * t15;
    let t789 = t130 * t2;
    let t790 = t4 * t88;
    let t791 = t789 * t790;
    let t794 = t128 * t97;
    let t795 = t794 * t728;
    let t804 = 0.619125e-2 * t784 * t145 - 0.123825e-1 * t788 * t791 - 0.619125e-2 * t144 * t795 - 0.53062222222222222221e-1 * t85 * t11 * t122 - 0.79593333333333333331e-1 * t85 * t148 * t745;
    (t788, t789, t791, t794, t795, t804)
}
