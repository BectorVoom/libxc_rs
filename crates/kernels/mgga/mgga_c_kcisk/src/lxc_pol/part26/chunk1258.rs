//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1258/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1258<F: Float>(t31944: F, t9301: F, t15268: F, t2677: F, t3934: F, t9318: F, t15217: F, t9307: F, t9314: F, t127: F, t15200: F, t43192: F, t2676: F, t43655: F, t31932: F, t31952: F) -> (F, F, F, F, F, F) {
    let t111314 = t9301 * t31944;
    let t111318 = t2677 * t3934 * t9318 * t15268;
    let t111321 = t15217 * t9314 * t9307;
    let t111326 = t3934 * t127 * t43192 * t15200;
    let t111327 = t43655 * t2676 * t111326;
    let t111329 = t31952 * t31932;
    (t111314, t111318, t111321, t111326, t111327, t111329)
}
