//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1244/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1244<F: Float>(t143: F, t30341: F, t30358: F, t30390: F, t30442: F, t3205: F, t10665: F, t10668: F, t10674: F, t10677: F, t10743: F, t151: F, t154: F, t157: F, t160: F, t163: F, t166: F, t169: F, t2081: F, t2109: F, t3233: F, t693: F, t707: F, t8684: F) -> (F, F, F, F) {
    let t145 = 0.135e1 < t143;
    let t30444 = t30341 + t30358 + t30390 + t30442;
    let t30445 = piecewise3(t145, t30444, 0.0);
    let t30448 = t3205 * t3205;
    let t30459 = -t10665 * t2081 / 6881280.0 - t10668 * t2081 / 13271040.0 + t3233 * t8684 / 0.10616832e9 + t10674 * t2081 / 0.21233664e9 + t10677 * t2081 / 412876800.0 + t151 * t10743 * t707 / 3.0 - t154 * t10743 * t707 / 24.0 + t157 * t10743 * t707 / 320.0 - t160 * t10743 * t707 / 5760.0 + t163 * t10743 * t707 / 129024.0 - t166 * t10743 * t707 / 3440640.0 + t169 * t10743 * t707 / 0.10616832e9 - t693 * t30445 / 18.0 + t163 * t30448 / 129024.0 - t166 * t30448 / 3440640.0 + t169 * t30448 / 0.10616832e9 - t2109 * t30448 / 0.37158912e10 + t151 * t30448 / 3.0;
    (t30444, t30445, t30448, t30459)
}
