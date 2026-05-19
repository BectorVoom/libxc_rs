//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 989/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk989<F: Float>(t13276: F, t1562: F, t4614: F, t13372: F, t1641: F, t13433: F, t4527: F, t13474: F, t597: F, t42349: F, t42366: F, t42369: F) -> (F, F, F, F, F, F, F) {
    let t46683 = F::cast_from(0.82820720060468819301e2_f64) * t1562 * t4614 * t13276;
    let t46688 = F::cast_from(0.46011511144704899612e1_f64) * t1641 * t13372;
    let t46691 = F::cast_from(0.36809208915763919689e2_f64) * t4527 * t4614 * t13433;
    let t46696 = F::cast_from(0.58281247449959539508e2_f64) * t597 * t4614 * t13474;
    let t46699 = F::cast_from(0.23005755572352449806e1_f64) * t42349;
    let t46703 = F::cast_from(0.59584149919750711116e-1_f64) * t42366;
    let t46704 = F::cast_from(0.59584149919750711116e-1_f64) * t42369;
    (t46683, t46688, t46691, t46696, t46699, t46703, t46704)
}
