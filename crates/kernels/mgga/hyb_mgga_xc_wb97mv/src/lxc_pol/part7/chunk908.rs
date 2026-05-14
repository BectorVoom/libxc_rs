//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 908/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk908<F: Float>(t43: F, t8266: F, t3046: F, t66: F, t69: F, t1911: F, t51: F, t1883: F, t3019: F, t3069: F, t3074: F, t3079: F, t564: F, t583: F, t587: F, t591: F, t595: F, t599: F, t603: F, t607: F, t611: F) -> (F, F) {
    let t45 = 0.135e1 < t43;
    let t8267 = piecewise3(t45, t8266, 0.0);
    let t8284 = t66 * t3046;
    let t8289 = t69 * t3046;
    let t8294 = t1911 * t3046;
    let t8299 = t51 * t3046;
    let t8304 = -t564 * t8267 / 18.0 + t587 * t8267 / 240.0 - t591 * t8267 / 4480.0 + t595 * t8267 / 103680.0 - t599 * t8267 / 2838528.0 + t603 * t8267 / 89456640.0 - t607 * t8267 / 0.31850496e10 + t611 * t8267 / 0.1263403008e12 - t8284 * t583 / 3440640.0 - t3069 * t1883 / 6881280.0 + t8289 * t583 / 0.10616832e9 + t3074 * t1883 / 0.21233664e9 - t8294 * t583 / 0.37158912e10 - t3079 * t1883 / 0.74317824e10 + t8299 * t583 / 3.0 + t3019 * t1883 / 6.0;
    (t8267, t8304)
}
