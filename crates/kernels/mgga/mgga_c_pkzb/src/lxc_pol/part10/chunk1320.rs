//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1320/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1320<F: Float>(t2815: F, t17616: F, t1911: F, t1938: F, t1940: F, t1955: F, t1977: F, t1979: F, t25597: F, t25642: F, t25818: F, t25829: F, t25831: F, t25834: F, t25836: F, t25838: F, t25840: F, t25842: F, t25844: F, t25846: F, t25857: F, t25859: F, t26083: F, t26096: F, t26109: F, t26122: F, t3581: F, t695: F, t703: F, t722: F, t9494: F) -> (F, F) {
    let t26134 = t2815 * t2815;
    let t26141 = -t25818 + 2.0 * t1911 * t9494 + 1.0 * t695 * (t26083 + t26096 + t26109 + t26122) * t703 + 0.32163958997385070134e2 * t17616 * t3581 + 0.34631718211362927518e2 * t1977 * t25597 * t1979 - t25829 - t25831 - t25834 - t25836 - t25838 - t25840 + t25842 - t25844 - t25846 - 0.19751673498613801407e-1 * t25642 + 0.64327917994770140268e2 * t1938 * t26134 * t1940 - 0.23392894490538584828e1 * t1955 * t25597 * t722 + t25857 - t25859;
    (t26134, t26141)
}
