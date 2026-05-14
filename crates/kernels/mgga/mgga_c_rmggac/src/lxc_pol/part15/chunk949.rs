//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 949/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk949<F: Float>(t7720: F, t9790: F, t46420: F, t7204: F, t46424: F, t7192: F, t46428: F, t8620: F, t46431: F, t8640: F, t10100: F, t236: F, t498: F, t7230: F, t7248: F, t321: F, t9188: F) -> (F, F, F, F, F, F, F) {
    let t47883 = t7720 * t9790;
    let t47885 = t7204 * t46420;
    let t47887 = t7192 * t46424;
    let t47889 = t8620 * t46428;
    let t47891 = t8640 * t46431;
    let t47898 = t7230 * t7248 * t236 * t10100 * t498;
    let t47903 = t7230 * t9188 * t236 * t10100 * t321;
    (t47883, t47885, t47887, t47889, t47891, t47898, t47903)
}
