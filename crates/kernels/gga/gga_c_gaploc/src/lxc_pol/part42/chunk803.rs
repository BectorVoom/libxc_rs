//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 803/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk803<F: Float>(t13372: F, t1641: F, t13433: F, t4527: F, t4614: F, t13474: F, t597: F, t42349: F, t42366: F, t42369: F, t42378: F, t11433: F, t1415: F, t7030: F, t11426: F, t9562: F) -> (F, F, F, F, F, F, F, F, F) {
    let t46688 = 0.46011511144704899612e1 * t1641 * t13372;
    let t46691 = 0.36809208915763919689e2 * t4527 * t4614 * t13433;
    let t46696 = 0.58281247449959539508e2 * t597 * t4614 * t13474;
    let t46699 = 0.23005755572352449806e1 * t42349;
    let t46703 = 0.59584149919750711116e-1 * t42366;
    let t46704 = 0.59584149919750711116e-1 * t42369;
    let t46705 = 0.25561950635947166451e0 * t42378;
    let t46707 = t1415 * t11433 * t7030;
    let t46708 = 0.14896037479937677779e-1 * t46707;
    let t46709 = t11426 * t9562;
    (t46688, t46691, t46696, t46699, t46703, t46704, t46705, t46708, t46709)
}
