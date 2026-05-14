//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 858/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk858<F: Float>(t46952: F, t568: F, t597: F, t600: F, t13821: F, t1628: F, t574: F, t13825: F, t13750: F, t1589: F, t557: F, t13829: F, t193: F, t524: F, t1: F, t46873: F, t544: F) -> (F, F, F, F, F, F) {
    let t48017 = 0.23005755572352449806e1 * t597 * t568 * t600 * t46952;
    let t48020 = 0.30674340763136599741e1 * t574 * t1628 * t13821;
    let t48023 = 0.30674340763136599741e1 * t597 * t1628 * t13825;
    let t48026 = 0.23833659967900284446e0 * t557 * t1589 * t13750;
    let t48029 = 0.35750489951850426669e0 * t524 * t13829 * t193;
    let t48032 = t544 * t46873 * t1;
    (t48017, t48020, t48023, t48026, t48029, t48032)
}
