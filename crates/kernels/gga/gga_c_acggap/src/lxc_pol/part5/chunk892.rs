//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 892/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk892<F: Float>(t3431: F, t4975: F, t5157: F, t13957: F, t537: F, t4878: F, t997: F, t4853: F, t4849: F, t1581: F, t3237: F, t3379: F, t1298: F, t435: F) -> (F, F, F, F, F, F, F, F, F) {
    let t15920 = t3431 * t4975;
    let t15922 = t3431 * t5157;
    let t15930 = t13957 * t537;
    let t15932 = t997 * t4878;
    let t15934 = t997 * t4853;
    let t15936 = t997 * t4849;
    let t15938 = t3237 * t1581;
    let t15945 = t3379 * t5157;
    let t15947 = t435 * t1298;
    (t15920, t15922, t15930, t15932, t15934, t15936, t15938, t15945, t15947)
}
