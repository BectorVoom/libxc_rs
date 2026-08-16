//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 948/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk948(t12068: f64, t2268: f64, t6320: f64, t6509: f64, t1063: f64, t11981: f64, t2343: f64, t6519: f64, t13755: f64, t535: f64, t13735: f64, t6313: f64) -> (f64, f64, f64, f64) {
    let t46900 = t2268 * t6320 * t12068 * t6509;
    let t46908 = t1063 * t2343 * t11981 * t6519;
    let t46912 = 0.28455006635676149599e-1_f64 * t2268 * t535 * t13755;
    let t46913 = t6313 * t13735;
    (t46900, t46908, t46912, t46913)
}
