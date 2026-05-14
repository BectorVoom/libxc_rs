//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 280/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk280<F: Float>(t1695: F, t1718: F, t1699: F, t1710: F, t1715: F, t1722: F, t633: F) -> (F, F, F, F) {
    let t1739 = 0.301925e0 * t1695;
    let t1742 = 0.16557e0 * t1718;
    let t1744 = 0.258925e1 * t1710 - t1739 - 0.301925e0 * t1699 + 0.16504875e0 * t1715 - t1742 - 0.16557e0 * t1722;
    let t1746 = 1.0 / t633;
    (t1739, t1742, t1744, t1746)
}
