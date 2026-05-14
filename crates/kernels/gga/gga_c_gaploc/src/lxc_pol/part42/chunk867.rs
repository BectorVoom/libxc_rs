//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 867/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk867<F: Float>(t14267: F, t14318: F, t14319: F, t14322: F, t14323: F, t14331: F, t14335: F, t1580: F, t1589: F, t1599: F, t1628: F, t1641: F, t41918: F, t46370: F, t46371: F, t46372: F, t47965: F, t47968: F, t47976: F, t47978: F, t47980: F, t47984: F, t49841: F, t49842: F, t531: F, t541: F, t557: F, t568: F, t569: F, t574: F, t597: F, t600: F) -> (F,) {
    let t50647 = 0.63904876589867916127e-1 * t41918 + 0.59584149919750711116e-1 * t47965 + 0.59584149919750711116e-1 * t47968 - t46370 - t46371 - t46372 + 0.59584149919750711116e-1 * t47976 + 0.59584149919750711116e-1 * t47978 - 0.59584149919750711116e-1 * t47980 - 0.59584149919750711116e-1 * t47984 + 0.30674340763136599741e1 * t597 * t1628 * t14322 - 0.35750489951850426669e0 * t557 * t531 * t49842 + 0.23005755572352449806e1 * t597 * t568 * t600 * t49841 - 0.35750489951850426669e0 * t1599 * t14331 - 0.23005755572352449806e1 * t574 * t568 * t569 * t49841 + 0.23005755572352449806e1 * t1580 * t14323 - 0.23005755572352449806e1 * t1641 * t14319 - 0.30674340763136599741e1 * t574 * t1628 * t14318 - 0.23833659967900284446e0 * t557 * t1589 * t14267 + 0.23833659967900284446e0 * t14335 * t541;
    (t50647,)
}
