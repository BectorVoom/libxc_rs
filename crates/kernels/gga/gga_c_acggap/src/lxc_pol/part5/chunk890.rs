//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 890/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk890<F: Float>(t1036: F, t1095: F, t1524: F, t398: F, t864: F, t1434: F, t3770: F, t1032: F, t4720: F, t4523: F, t1008: F, t5108: F, t14106: F, t542: F, t13957: F, t532: F) -> (F, F, F, F, F, F, F, F) {
    let t15826 = t1036 * t398 * t1095 * t1524 * t864;
    let t15828 = t3770 * t1434;
    let t15830 = t1032 * t4720;
    let t15832 = t1032 * t4523;
    let t15841 = t1008 * t5108;
    let t15849 = t14106 * t542;
    let t15851 = t13957 * t542;
    let t15853 = t13957 * t532;
    (t15826, t15828, t15830, t15832, t15841, t15849, t15851, t15853)
}
