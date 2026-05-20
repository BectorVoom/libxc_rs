//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2122/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2122<F: Float>(t1043: F, t4772: F, t1045: F, t3117: F, t1086: F, t4746: F, t3090: F, t15822: F, t3160: F, t1065: F, t2852: F, t1469: F, t2251: F) -> (F, F, F, F, F, F, F, F) {
    let t15920 = t4772 * t1043;
    let t15921 = t15920 * t1045;
    let t15922 = t3117 * t15921;
    let t15925 = t4746 * t1086;
    let t15926 = t15925 * t3090;
    let t15932 = t15822 * t3160;
    let t15935 = t1065 * t2852;
    let t15936 = t1469 * t2251;
    (t15920, t15921, t15922, t15925, t15926, t15932, t15935, t15936)
}
