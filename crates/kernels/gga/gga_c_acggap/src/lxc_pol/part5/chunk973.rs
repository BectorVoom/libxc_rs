//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 973/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk973<F: Float>(t1032: F, t4720: F, t4523: F, t1008: F, t5108: F, t14106: F, t542: F, t13957: F, t532: F, t4396: F, t5138: F, t5143: F) -> (F, F, F, F, F, F, F, F) {
    let t15830 = t1032 * t4720;
    let t15832 = t1032 * t4523;
    let t15841 = t1008 * t5108;
    let t15849 = t14106 * t542;
    let t15851 = t13957 * t542;
    let t15853 = t13957 * t532;
    let t15855 = t4396 * t5138;
    let t15871 = t4396 * t5143;
    (t15830, t15832, t15841, t15849, t15851, t15853, t15855, t15871)
}
