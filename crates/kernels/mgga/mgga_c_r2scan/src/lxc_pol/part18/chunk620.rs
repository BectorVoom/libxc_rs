//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 620/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk620<F: Float>(t322: F, t1048: F, t3618: F, t499: F, t2867: F, t3263: F, t3275: F, t1010: F, t3358: F, t1070: F, t2378: F, t1276: F, t3357: F, t3368: F) -> (F, F, F, F, F) {
    let t324 = 0.0 < t322;
    let t3620 = t1048 * t499 * t3618;
    let t3621 = t3620 / 4.0;
    let t3623 = t3275 * t3263 * t2867;
    let t3624 = t3623 / 4.0;
    let t3625 = t3358 * t1010;
    let t3627 = t2378 * t1070;
    let t3629 = t1070 * t1010;
    let t3630 = t1276 * t3629;
    let t3632 = t3357 + t3625 / 8.0 - t3627 / 8.0 + t3630 / 4.0 + t3368;
    let t3633 = piecewise3(t324, 0.0, t3632);
    (t3621, t3624, t3629, t3632, t3633)
}
