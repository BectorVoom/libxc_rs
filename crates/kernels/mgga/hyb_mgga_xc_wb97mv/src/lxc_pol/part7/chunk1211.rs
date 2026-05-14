//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1211/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1211<F: Float>(t10311: F, t10314: F, t10364: F, t10383: F, t10386: F, t10392: F, t1842: F, t1883: F, t1911: F, t3019: F, t3049: F, t3079: F, t3897: F, t3899: F, t51: F, t54: F, t57: F, t583: F, t60: F, t63: F, t66: F, t69: F, t8267: F) -> (F,) {
    let t29245 = -t3079 * t8267 / 0.37158912e10 - t10311 * t1883 / 0.74317824e10 - 2.0 / 3.0 * t10314 * t1883 - t54 * t10364 * t583 / 24.0 + t57 * t10364 * t583 / 320.0 - t60 * t10364 * t583 / 5760.0 + t63 * t10364 * t583 / 129024.0 - t66 * t10364 * t583 / 3440640.0 + t69 * t10364 * t583 / 0.10616832e9 - t1911 * t10364 * t583 / 0.37158912e10 + t51 * t10364 * t583 / 3.0 + t3019 * t8267 / 3.0 + t10383 * t1883 / 6.0 + t10386 * t1883 / 8.0 - t3049 * t8267 / 24.0 - t10392 * t1883 / 48.0 + 10.0 / 3.0 * t3897 * t1842 - 2.0 / 3.0 * t3899 * t1842;
    (t29245,)
}
