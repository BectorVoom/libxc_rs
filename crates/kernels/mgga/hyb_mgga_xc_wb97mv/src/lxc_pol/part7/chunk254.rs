//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 254/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk254<F: Float>(t238: F, t243: F, t800: F, t226: F, t778: F, t242: F, t780: F, t791: F, t793: F, t796: F) -> (F, F, F, F, F) {
    let t802 = t238 * t800 * t243;
    let t803 = 0.82156666666666666667e-1 * t802;
    let t804 = t226 * t778;
    let t806 = t238 * t242 * t804;
    let t808 = 0.1898925e1 * t791 - t793 + 0.8969e0 * t780 + 0.3071625e0 * t796 - t803 + 0.24647e0 * t806;
    (t802, t803, t804, t806, t808)
}
