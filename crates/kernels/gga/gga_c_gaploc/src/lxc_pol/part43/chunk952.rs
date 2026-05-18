//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 952/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk952<F: Float>(t11977: F, t2268: F, t2349: F, t1063: F, t13829: F, t448: F, t13732: F, t6313: F, t105: F, t169: F, t172: F, t452: F, t46952: F) -> (F, F, F, F) {
    let t46970 = t2268 * t11977 * t2349;
    let t46979 = F::new(0.28455006635676149599e-1) * t1063 * t13829 * t448;
    let t46980 = t6313 * t13732;
    let t46991 = F::new(0.28455006635676149599e-1) * t105 * t452 * t46952 * t169 * t172;
    (t46970, t46979, t46980, t46991)
}
