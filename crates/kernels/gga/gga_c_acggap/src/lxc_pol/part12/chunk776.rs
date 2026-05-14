//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 776/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk776<F: Float>(t322: F, t945: F, t174: F, t361: F, t157: F, t406: F, t864: F, t1016: F, t965: F, t1487: F, t435: F, t929: F, t145: F, t4875: F, t1101: F, t360: F) -> (F, F, F, F, F, F, F, F) {
    let t15407 = t945 * t322;
    let t15695 = t361 * t174;
    let t15758 = t864 * t406 * t157;
    let t15897 = t965 * t1016;
    let t15995 = t435 * t1487;
    let t16020 = t322 * t929 * t157;
    let t16314 = t4875 * t145;
    let t16325 = t1101 * t360;
    (t15407, t15695, t15758, t15897, t15995, t16020, t16314, t16325)
}
