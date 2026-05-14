//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 843/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk843<F: Float>(t58: F, t875: F, t423: F, t122: F, t597: F, t10673: F, t3308: F, t870: F) -> (F, F, F, F, F, F) {
    let t10674 = t875 * t58;
    let t10675 = t10674 * t423;
    let t10676 = t597 * t122;
    let t10677 = t10675 * t10676;
    let t10678 = t10673 * t10677;
    let t10680 = t870 * t3308;
    (t10674, t10675, t10676, t10677, t10678, t10680)
}
