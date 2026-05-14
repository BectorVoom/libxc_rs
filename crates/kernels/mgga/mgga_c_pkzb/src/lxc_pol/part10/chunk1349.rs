//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1349/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1349<F: Float>(t6137: F, t9847: F, t6317: F, t9850: F, t18790: F, t9853: F, t9856: F, t9860: F, t2197: F, t851: F, t9838: F, t26851: F, t26854: F, t26857: F, t26859: F, t26861: F, t26863: F, t26865: F) -> (F, F, F, F, F, F, F) {
    let t26867 = 12.0 * t6137 * t9847;
    let t26869 = 8.0 * t6317 * t9850;
    let t26871 = 0.1929837539843104208e3 * t18790 * t9853;
    let t26873 = 4.0 * t6317 * t9856;
    let t26875 = 0.32163958997385070134e2 * t6137 * t9860;
    let t26878 = 4.0 * t2197 * t9838 * t851;
    let t26879 = -t26851 + t26854 + t26857 - t26859 + t26861 - t26863 + t26865 + t26867 - t26869 - t26871 - t26873 + t26875 - t26878;
    (t26867, t26869, t26871, t26873, t26875, t26878, t26879)
}
