//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 898/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk898<F: Float>(t195: F, t6750: F, t2531: F, t642: F, t1821: F, t998: F, t1062: F, t1469: F, t2724: F, t462: F, t1020: F, t568: F) -> (F, F, F, F, F, F) {
    let t6751 = t6750 * t195;
    let t6752 = t2531 * t642;
    let t6754 = t998 * t1821;
    let t6755 = t1469 * t1062;
    let t6756 = t462 * t2724;
    let t6758 = t1020 * t568;
    (t6751, t6752, t6754, t6755, t6756, t6758)
}
