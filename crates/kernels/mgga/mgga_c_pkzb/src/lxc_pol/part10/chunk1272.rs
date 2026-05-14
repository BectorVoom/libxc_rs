//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1272/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1272<F: Float>(t2706: F, t568: F, t1020: F, t1673: F, t1535: F, t16701: F, t16873: F, t16875: F, t24633: F, t24634: F, t24635: F, t24636: F, t2537: F, t2718: F, t6758: F, t7181: F, t7197: F, t7201: F, t8758: F) -> (F,) {
    let t25005 = t568 * t2706;
    let t25015 = t1020 * t1673;
    let t25019 = -12.0 * t1535 * t25005 * t2537 + 12.0 * t1535 * t25015 * t7197 - 12.0 * t1535 * t7181 * t8758 + 24.0 * t2718 * t6758 * t7201 + t16701 + t16873 - t16875 - t24633 + t24634 + t24635 - t24636;
    (t25019,)
}
