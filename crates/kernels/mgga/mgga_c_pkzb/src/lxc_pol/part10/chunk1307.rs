//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1307/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1307<F: Float>(t17326: F, t3525: F, t3551: F, t5766: F, t1850: F, t9390: F, t20975: F, t2873: F, t730: F, t25821: F, t25823: F, t25827: F, t25829: F, t25831: F, t25834: F, t25836: F, t25838: F, t25840: F) -> (F, F, F, F, F) {
    let t25842 = 2.0 * t17326 * t3525;
    let t25844 = 1.0 * t5766 * t3551;
    let t25846 = 2.0 * t1850 * t9390;
    let t25849 = 0.34631718211362927518e2 * t730 * t2873 * t20975;
    let t25850 = -t25821 + t25823 - t25827 + t25829 + t25831 + t25834 + t25836 + t25838 + t25840 - t25842 + t25844 + t25846 - t25849;
    (t25842, t25844, t25846, t25849, t25850)
}
