//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1061/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1061<F: Float>(t11029: F, t238: F, t242: F, t11009: F, t11011: F, t11016: F, t11020: F, t11023: F, t11027: F, t6817: F, t6847: F, t8958: F, t9101: F, t9102: F, t11007: F, t809: F) -> (F, F, F) {
    let t11031 = t238 * t242 * t11029;
    let t11033 = 0.15358125e0 * t11009 + 0.3071625e0 * t11011 - t6847 + 0.27385555555555555556e0 * t6817 + 0.5477111111111111111e0 * t8958 - t9101 - t9102 - 0.16431333333333333333e0 * t11016 + 0.49294e0 * t11020 - 0.16431333333333333333e0 * t11023 + 0.24647e0 * t11027 + 0.24647e0 * t11031;
    let t11034 = t11007 + t11033;
    let t11035 = t11034 * t809;
    (t11031, t11034, t11035)
}
