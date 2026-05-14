//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1272/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1272<F: Float>(t17330: F, t459: F, t225: F, t480: F, t1256: F, t5258: F, t5262: F, t1804: F, t3655: F, t1786: F, t1260: F, t12987: F, t1774: F, t3568: F, t247: F, t3719: F) -> (F, F, F, F, F, F, F, F, F) {
    let t17331 = t17330 * t459;
    let t17332 = t17331 * t225;
    let t17333 = t17332 * t480;
    let t17337 = 0.15244095330869239812e-2 * t5258 * t1256;
    let t17339 = 0.28582678745379824648e-3 * t5262 * t1256;
    let t17340 = t1804 * t3655;
    let t17342 = t1786 * t3655;
    let t17344 = t12987 * t1260;
    let t17345 = t1774 * t3568;
    let t17347 = t247 * t3719 * t17345;
    (t17331, t17333, t17337, t17339, t17340, t17342, t17344, t17345, t17347)
}
