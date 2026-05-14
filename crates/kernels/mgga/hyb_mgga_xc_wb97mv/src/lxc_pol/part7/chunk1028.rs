//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1028/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1028<F: Float>(t43: F, t10363: F, t3894: F, t51: F, t3877: F, t591: F, t10303: F, t10306: F, t10311: F, t10314: F, t3019: F, t3046: F, t3079: F, t564: F, t583: F, t587: F, t595: F, t599: F, t603: F, t607: F, t611: F) -> (F, F, F, F) {
    let t45 = 0.135e1 < t43;
    let t10364 = piecewise3(t45, t10363, 0.0);
    let t10383 = t51 * t3894;
    let t10386 = t591 * t3877;
    let t10389 = t10303 * t583 / 0.21233664e9 + t10306 * t583 / 412876800.0 - t3079 * t3046 / 0.37158912e10 - t10311 * t583 / 0.74317824e10 - 2.0 / 3.0 * t10314 * t583 + t587 * t10364 / 240.0 - t591 * t10364 / 4480.0 + t595 * t10364 / 103680.0 - t599 * t10364 / 2838528.0 + t603 * t10364 / 89456640.0 - t607 * t10364 / 0.31850496e10 + t611 * t10364 / 0.1263403008e12 - t564 * t10364 / 18.0 + t3019 * t3046 / 3.0 + t10383 * t583 / 6.0 + t10386 * t583 / 8.0;
    (t10364, t10383, t10386, t10389)
}
