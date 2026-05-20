//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3145/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3145<F: Float>(t12627: F, t1811: F, t12657: F, t1208: F, t17330: F, t487: F, t1269: F, t17306: F, t1209: F, t1270: F, t3566: F, t17331: F) -> (F, F, F, F, F, F, F, F) {
    let t56393 = t12627 * t1811;
    let t56396 = t12657 * t1811;
    let t56412 = t17330 * t1208;
    let t56413 = t56412 * t487;
    let t56416 = t17306 * t1269;
    let t56419 = t1209 * t1270;
    let t56432 = t3566 * t1270;
    let t56486 = t17331 * t487;
    (t56393, t56396, t56412, t56413, t56416, t56419, t56432, t56486)
}
