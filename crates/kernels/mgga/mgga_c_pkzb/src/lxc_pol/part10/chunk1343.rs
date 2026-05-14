//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1343/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1343<F: Float>(t1306: F, t17753: F, t2153: F, t25818: F, t25821: F, t25823: F, t25827: F, t25829: F, t25831: F, t25834: F, t25836: F, t25838: F, t25840: F, t25842: F, t25844: F, t25846: F, t3702: F) -> (F,) {
    let t26802 = -6.0 * t1306 * t17753 * t2153 * t3702 + t25818 - t25821 + t25823 - t25827 + t25829 + t25831 + t25834 + t25836 + t25838 + t25840 - t25842 + t25844 + t25846;
    (t26802,)
}
