//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1303/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1303<F: Float>(t3507: F, t2492: F, t2224: F, t238: F, t4314: F, t941: F, t9334: F, t9337: F, t31811: F, t946: F, t23132: F, t23135: F, t23183: F, t23279: F, t27153: F, t27156: F, t27159: F) -> (F, F, F, F, F, F, F) {
    let t31840 = t3507 * t3507;
    let t31841 = t2492 * t31840;
    let t31844 = t238 * t2224 * t4314;
    let t31850 = t941 * t3507;
    let t31851 = t9334 * t31850;
    let t31853 = t9337 * t31850;
    let t31856 = t946 * t31811;
    let t31858 = 0.39862222222222222223e0 * t23183 + 0.3071625e0 * t31841 + 0.27385555555555555555e0 * t31844 + 0.10954222222222222222e1 * t27153 + 0.10954222222222222222e1 * t27156 - 0.14605629629629629629e1 * t27159 + 0.27385555555555555556e0 * t23132 + t23279 + 0.5696775e1 * t31851 - 0.3071625e0 * t31853 + 0.27385555555555555556e0 * t23135 + 0.3071625e0 * t31856;
    (t31840, t31841, t31844, t31851, t31853, t31856, t31858)
}
