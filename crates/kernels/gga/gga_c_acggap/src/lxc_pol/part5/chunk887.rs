//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 887/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk887<F: Float>(t1165: F, t12936: F, t3044: F, t540: F, t4254: F, t8887: F, t174: F, t361: F, t1181: F, t3361: F, t3754: F, t530: F, t3730: F, t14056: F, t4912: F, t3621: F, t4640: F) -> (F, F, F, F, F, F, F) {
    let t15675 = t12936 * t1165 * t540 * t3044;
    let t15690 = t4254 * t8887;
    let t15695 = t361 * t174;
    let t15710 = t3361 * t1181 * t530 * t3754;
    let t15714 = t3361 * t1181 * t530 * t3730;
    let t15733 = t14056 * t4912;
    let t15746 = t3621 * t4640;
    (t15675, t15690, t15695, t15710, t15714, t15733, t15746)
}
