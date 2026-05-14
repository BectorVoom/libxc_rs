//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1270/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1270<F: Float>(t135: F, t1535: F, t1536: F, t16626: F, t16631: F, t1673: F, t1692: F, t1816: F, t19800: F, t24623: F, t24624: F, t24964: F, t24973: F, t2536: F, t2718: F, t3396: F, t3495: F, t568: F, t6763: F, t8779: F, t8817: F, t9112: F) -> (F,) {
    let t24986 = 6.0 * t135 * t3396 * t6763 + 6.0 * t1535 * t1536 * t8817 + 3.0 * t1535 * t1692 * t9112 - 6.0 * t1673 * t24973 * t2536 + 2.0 * t1816 * t2536 * t8779 + 12.0 * t24964 * t2718 * t568 + 12.0 * t19800 * t3495 + t16626 - t16631 + t24623 + t24624;
    (t24986,)
}
