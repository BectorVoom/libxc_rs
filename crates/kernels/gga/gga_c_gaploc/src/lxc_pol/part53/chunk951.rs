//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 951/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk951<F: Float>(t46952: F, t493: F, t105: F, t492: F, t13729: F, t6313: F, t13732: F, t6305: F, t12000: F, t555: F, t2268: F, t888: F) -> (F, F, F, F, F, F) {
    let t46953 = t493 * t46952;
    let t46956 = F::new(0.28455006635676149599e-1) * t105 * t492 * t46953;
    let t46961 = t6313 * t13729;
    let t46963 = t6305 * t13732;
    let t46965 = t555 * t12000;
    let t46967 = t2268 * t46965 * t888;
    (t46953, t46956, t46961, t46963, t46965, t46967)
}
