//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 950/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk950<F: Float>(t447: F, t46867: F, t1063: F, t1064: F, t11981: F, t2293: F, t2268: F, t2343: F, t39656: F, t39657: F, t42111: F, t42113: F, t42114: F, t42117: F, t42118: F, t42119: F, t42120: F, t42121: F, t42122: F) -> (F, F, F, F, F) {
    let t46941 = t46867 * t447;
    let t46944 = F::cast_from(0.28455006635676149599e-1_f64) * t1063 * t1064 * t46941;
    let t46945 = t11981 * t2293;
    let t46947 = t2268 * t2343 * t46945;
    let t46952 = t42111 - t42113 + t42114 / F::new(2.0) + t39656 - t39657 + t42117 + t42118 - t42119 + t42120 - t42121 - t42122;
    (t46941, t46944, t46945, t46947, t46952)
}
