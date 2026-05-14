//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1081/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1081<F: Float>(t16710: F, t16712: F, t1256: F, t5258: F, t5262: F, t1804: F, t3655: F, t1786: F, t1260: F, t12987: F, t15687: F, t3623: F, t3782: F, t1263: F, t1794: F, t372: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t17320 = 0.11111111111111111111e-1 * t16710;
    let t17321 = 0.55555555555555555556e-2 * t16712;
    let t17337 = 0.15244095330869239812e-2 * t5258 * t1256;
    let t17339 = 0.28582678745379824648e-3 * t5262 * t1256;
    let t17340 = t1804 * t3655;
    let t17342 = t1786 * t3655;
    let t17344 = t12987 * t1260;
    let t17350 = t3623 * t15687;
    let t17351 = t3782 * t17350;
    let t17352 = t1263 * t1794;
    let t17353 = t372 * t17352;
    (t17320, t17321, t17337, t17339, t17340, t17342, t17344, t17350, t17351, t17353)
}
