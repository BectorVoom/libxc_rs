//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 891/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk891<F: Float>(t4396: F, t5138: F, t5143: F, t1005: F, t5089: F, t13635: F, t527: F, t3371: F, t4198: F, t4452: F, t4384: F, t4389: F, t12813: F, t4967: F, t13084: F, t4971: F) -> (F, F, F, F, F, F, F, F, F) {
    let t15855 = t4396 * t5138;
    let t15871 = t4396 * t5143;
    let t15891 = t1005 * t5089;
    let t15902 = t13635 * t527;
    let t15905 = t4198 * t3371;
    let t15906 = t15905 * t4452;
    let t15914 = t4389 * t4384;
    let t15916 = t12813 * t4967;
    let t15918 = t13084 * t4971;
    (t15855, t15871, t15891, t15902, t15905, t15906, t15914, t15916, t15918)
}
