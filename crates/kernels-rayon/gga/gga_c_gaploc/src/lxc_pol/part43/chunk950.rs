//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 950/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk950(t447: f64, t46867: f64, t1063: f64, t1064: f64, t11981: f64, t2293: f64, t2268: f64, t2343: f64, t39656: f64, t39657: f64, t42111: f64, t42113: f64, t42114: f64, t42117: f64, t42118: f64, t42119: f64, t42120: f64, t42121: f64, t42122: f64) -> (f64, f64, f64, f64, f64) {
    let t46941 = t46867 * t447;
    let t46944 = 0.28455006635676149599e-1_f64 * t1063 * t1064 * t46941;
    let t46945 = t11981 * t2293;
    let t46947 = t2268 * t2343 * t46945;
    let t46952 = t42111 - t42113 + t42114 / 2.0_f64 + t39656 - t39657 + t42117 + t42118 - t42119 + t42120 - t42121 - t42122;
    (t46941, t46944, t46945, t46947, t46952)
}
