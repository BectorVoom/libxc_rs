//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1004/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1004<F: Float>(t1818: F, t1970: F, t209: F, t236: F, t476: F, t9210: F, t10082: F, t495: F, t7230: F, t7248: F, t1916: F, t2144: F) -> (F, F, F) {
    let t46969 = t1970 * t9210 * t236 * t1818 * t476 * t209;
    let t46974 = t7230 * t7248 * t236 * t10082 * t495;
    let t46976 = t1916 * t2144;
    (t46969, t46974, t46976)
}
