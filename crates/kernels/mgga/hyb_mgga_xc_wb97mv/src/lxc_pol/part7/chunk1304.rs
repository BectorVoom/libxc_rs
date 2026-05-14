//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1304/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1304<F: Float>(t2480: F, t31840: F, t2224: F, t238: F, t4310: F, t3487: F, t242: F, t27021: F, t27024: F, t27027: F, t27207: F, t27210: F, t27213: F, t31779: F, t31782: F, t31810: F) -> (F, F, F, F) {
    let t31860 = t2480 * t31840;
    let t31869 = t238 * t2224 * t4310;
    let t31871 = t3487 * t3487;
    let t31873 = t238 * t242 * t31871;
    let t31878 = -0.1898925e1 * t31860 - 0.1860237037037037037e1 * t27021 + 0.15944888888888888889e1 * t27024 - 0.59793333333333333334e0 * t27027 - 0.32862666666666666666e0 * t27207 - 0.65725333333333333332e0 * t27210 - 0.32862666666666666666e0 * t27213 + 0.27385555555555555555e0 * t31869 + 0.49294e0 * t31873 + 0.39862222222222222223e0 * t31779 - 0.59793333333333333334e0 * t31782 + 0.8969e0 * t31810;
    (t31860, t31869, t31873, t31878)
}
