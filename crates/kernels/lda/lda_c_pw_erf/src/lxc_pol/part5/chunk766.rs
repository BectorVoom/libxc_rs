//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 766/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk766<F: Float>(t1319: F, t7426: F, t571: F, t2017: F, t7418: F, t2411: F, t811: F, t1318: F, t833: F, t3832: F, t6270: F, t743: F, t3867: F, t7630: F, t7663: F, t7678: F, t7682: F, t7686: F, t7690: F, t7694: F, t7697: F, t7700: F, t7704: F, t7708: F, t7712: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t7713 = t1319 * t7426;
    let t7715 = 8.0 / 15.0 * t571 * t7713;
    let t7716 = t2017 * t7418;
    let t7718 = 4.0 / 9.0 * t571 * t7716;
    let t7719 = t2411 * t811;
    let t7720 = t2017 * t7719;
    let t7722 = 8.0 / 9.0 * t1318 * t7720;
    let t7723 = t2411 * t833;
    let t7724 = t3832 * t7723;
    let t7726 = 4.0 / 9.0 * t571 * t7724;
    let t7727 = t6270 * t743;
    let t7728 = t3867 * t7727;
    let t7730 = 8.0 / 15.0 * t571 * t7728;
    let t7731 = t7630 + t7663 + t7678 - t7682 + t7686 + t7690 - t7694 - t7697 + t7700 + t7704 - t7708 - t7712 - t7715 + t7718 + t7722 - t7726 + t7730;
    (t7713, t7715, t7716, t7718, t7719, t7720, t7722, t7723, t7724, t7726, t7727, t7728, t7730, t7731)
}
