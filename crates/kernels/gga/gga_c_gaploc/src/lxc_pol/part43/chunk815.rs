//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 815/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk815<F: Float>(t12068: F, t2268: F, t6320: F, t6509: F, t1063: F, t11981: F, t2343: F, t6519: F, t13755: F, t535: F, t13735: F, t6313: F, t46867: F, t475: F, t1064: F, t13749: F, t599: F) -> (F, F, F, F, F, F, F) {
    let t46900 = t2268 * t6320 * t12068 * t6509;
    let t46908 = t1063 * t2343 * t11981 * t6519;
    let t46912 = 0.28455006635676149599e-1 * t2268 * t535 * t13755;
    let t46913 = t6313 * t13735;
    let t46915 = t46867 * t475;
    let t46918 = 0.85365019907028448797e-1 * t2268 * t1064 * t46915;
    let t46919 = t599 * t13749;
    (t46900, t46908, t46912, t46913, t46915, t46918, t46919)
}
