//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 851/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk851<F: Float>(t46707: F, t11426: F, t9562: F, t11318: F, t1445: F, t2293: F, t574: F, t13475: F, t1580: F, t1000: F, t13359: F, t34157: F, t40449: F, t40517: F, t42316: F, t42341: F, t44329: F, t44560: F, t4614: F, t46683: F, t46688: F, t46691: F, t46696: F, t46699: F, t46703: F, t46704: F, t46705: F, t567: F, t597: F) -> (F,) {
    let t46708 = 0.14896037479937677779e-1 * t46707;
    let t46709 = t11426 * t9562;
    let t46715 = 0.92023022289409799224e1 * t574 * t1445 * t11318 * t2293;
    let t46717 = 0.43710935587469654631e2 * t1580 * t13475;
    let t46721 = -t46683 - 0.12269736305254639897e2 * t574 * t4614 * t13359 - t46688 + t46691 + 0.47667319935800568892e0 * t1000 * t34157 + t46696 - 0.59584149919750711116e-1 * t42316 + 0.63904876589867916126e-1 * t40449 + t42341 - t46699 + 0.23005755572352449806e2 * t597 * t1445 * t44560 + t46703 + t46704 + t46705 - t46708 - 0.10427226235956374446e0 * t46709 + 0.2556195063594716645e0 * t40517 - t46715 + t46717 + 0.23005755572352449806e1 * t567 * t1445 * t44329;
    (t46721,)
}
