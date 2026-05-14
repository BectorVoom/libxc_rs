//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1304/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1304<F: Float>(t1862: F, t3532: F, t5547: F, t1867: F, t9137: F, t25782: F, t25785: F, t25788: F, t25790: F, t25793: F, t25795: F, t25797: F, t25799: F, t25802: F, t25804: F, t25729: F, t25764: F, t25780: F, t664: F, t684: F) -> (F, F, F) {
    let t25807 = t5547 * t3532 * t1862;
    let t25809 = t9137 * t1867;
    let t25811 = -0.3560484375e1 * t25782 + 0.142419375e1 * t25785 + 0.1151859375e0 * t25788 - 0.1898925e1 * t25790 - 0.1898925e1 * t25793 - 0.9494625e0 * t25795 - 0.76790625e-1 * t25797 + 0.3071625e0 * t25799 + 0.3071625e0 * t25802 + 0.15358125e0 * t25804 - 0.76790625e-1 * t25807 + 0.142419375e1 * t25809;
    let t25816 = 1.0 * t664 * (t25729 + t25764 + t25780 + t25811) * t684;
    (t25807, t25809, t25816)
}
