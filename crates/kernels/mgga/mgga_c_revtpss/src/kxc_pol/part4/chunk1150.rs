//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1150/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1150<F: Float>(t15885: F, t341: F, t225: F, t366: F, t1058: F, t4794: F, t1651: F, t3151: F, t3155: F, t3117: F, t3162: F, t11243: F, t72: F, t3088: F, t12078: F, t11249: F, t1668: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t15886 = t15885 * t341;
    let t15887 = t15886 * t225;
    let t15888 = t15887 * t366;
    let t15892 = 0.15244095330869239812e-2 * t4794 * t1058;
    let t15893 = t1651 * t3151;
    let t15894 = t15893 * t3155;
    let t15895 = t3117 * t15894;
    let t15898 = t15893 * t3162;
    let t15899 = t3117 * t15898;
    let t15904 = t11243 * t72;
    let t15905 = t3088 * t15904;
    let t15906 = t12078 * t15905;
    let t15907 = t1668 * t11249;
    (t15886, t15888, t15892, t15893, t15895, t15899, t15904, t15905, t15906, t15907)
}
