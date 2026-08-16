//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 951/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk951(t46952: f64, t493: f64, t105: f64, t492: f64, t13729: f64, t6313: f64, t13732: f64, t6305: f64, t12000: f64, t555: f64, t2268: f64, t888: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t46953 = t493 * t46952;
    let t46956 = 0.28455006635676149599e-1_f64 * t105 * t492 * t46953;
    let t46961 = t6313 * t13729;
    let t46963 = t6305 * t13732;
    let t46965 = t555 * t12000;
    let t46967 = t2268 * t46965 * t888;
    (t46953, t46956, t46961, t46963, t46965, t46967)
}
