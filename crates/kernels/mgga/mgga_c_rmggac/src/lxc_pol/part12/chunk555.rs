//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 555/951 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk555<F: Float>(t7624: F, t7666: F, t515: F, t235: F, t665: F, t848: F, t884: F, t1243: F, t128: F, t118: F, t2001: F, t675: F, t1987: F, t2191: F, t1268: F, t1986: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7667 = t7624 + t7666;
    let t7668 = t515 * t7667;
    let t7669 = t235 * t7668;
    let t7670 = 0.19957069503106347607e-1 * t7669;
    let t7672 = t665 * t848;
    let t7673 = t884 * t7672;
    let t7674 = 0.59871208509319042821e-1 * t7673;
    let t7675 = t128 * t1243;
    let t7676 = t118 * t7675;
    let t7677 = t2001 * t7676;
    let t7678 = t675 * t7677;
    let t7679 = 0.42564599893297839398e-5 * t7678;
    let t7680 = t2191 * t1987;
    let t7681 = 0.25538759935978703638e-4 * t7680;
    let t7682 = t1986 * t1268;
    (t7667, t7668, t7670, t7672, t7674, t7677, t7679, t7681, t7682)
}
