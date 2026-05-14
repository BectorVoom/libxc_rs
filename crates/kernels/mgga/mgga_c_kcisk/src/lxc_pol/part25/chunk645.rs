//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 645/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk645<F: Float>(t1814: F, t6790: F, t4638: F, t4691: F, t6756: F, t6761: F, t6766: F, t6769: F, t587: F, t1644: F, t2368: F, t1665: F, t2382: F, t4699: F, t1663: F, t4704: F) -> (F, F, F, F, F, F, F, F) {
    let t6791 = t1814 * t6790;
    let t6799 = t4691 + 0.5936111111111111111e-2 * t4638 + 0.5936111111111111111e-2 * t6756 - 0.11872222222222222222e-1 * t6761 + 0.35616666666666666666e-1 * t6766 + 0.35616666666666666666e-1 * t6769;
    let t6801 = 0.62182e-1 * t6799 * t587;
    let t6802 = t2368 * t1644;
    let t6804 = 1.0 * t6802 * t1665;
    let t6806 = 1.0 * t4699 * t2382;
    let t6807 = t2382 * t1663;
    let t6809 = 2.0 * t4704 * t6807;
    (t6791, t6799, t6801, t6802, t6804, t6806, t6807, t6809)
}
