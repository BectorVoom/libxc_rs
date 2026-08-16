//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 991/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk991(t1000: f64, t13359: f64, t1445: f64, t34157: f64, t40449: f64, t40517: f64, t42316: f64, t42341: f64, t44329: f64, t44560: f64, t4614: f64, t46683: f64, t46688: f64, t46691: f64, t46696: f64, t46699: f64, t46703: f64, t46704: f64, t46705: f64, t46708: f64, t46709: f64, t46715: f64, t46717: f64, t567: f64, t574: f64, t597: f64) -> f64 {
    let t46721 = -t46683 - 0.12269736305254639897e2_f64 * t574 * t4614 * t13359 - t46688 + t46691 + 0.47667319935800568892e0_f64 * t1000 * t34157 + t46696 - 0.59584149919750711116e-1_f64 * t42316 + 0.63904876589867916126e-1_f64 * t40449 + t42341 - t46699 + 0.23005755572352449806e2_f64 * t597 * t1445 * t44560 + t46703 + t46704 + t46705 - t46708 - 0.10427226235956374446e0_f64 * t46709 + 0.2556195063594716645e0_f64 * t40517 - t46715 + t46717 + 0.23005755572352449806e1_f64 * t567 * t1445 * t44329;
    t46721
}
