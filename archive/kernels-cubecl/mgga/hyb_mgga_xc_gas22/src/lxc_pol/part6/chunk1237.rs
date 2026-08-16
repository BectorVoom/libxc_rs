//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1237/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1237<F: Float>(t1351: F, t6682: F, t2289: F, t3396: F, t1363: F, t6640: F, t1358: F, t6710: F, t6683: F, t1370: F, t2312: F, t839: F, t8753: F) -> (F, F, F, F, F, F, F) {
    let t24816 = t1351 * t6682;
    let t24819 = t3396 * t2289;
    let t24822 = t1363 * t6640;
    let t24825 = t6710 * t1358;
    let t24829 = t6683 * t1358;
    let t24832 = t2312 * t1370;
    let t24842 = t8753 * t839;
    (t24816, t24819, t24822, t24825, t24829, t24832, t24842)
}
