//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 989/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk989(t13276: f64, t1562: f64, t4614: f64, t13372: f64, t1641: f64, t13433: f64, t4527: f64, t13474: f64, t597: f64, t42349: f64, t42366: f64, t42369: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t46683 = 0.82820720060468819301e2_f64 * t1562 * t4614 * t13276;
    let t46688 = 0.46011511144704899612e1_f64 * t1641 * t13372;
    let t46691 = 0.36809208915763919689e2_f64 * t4527 * t4614 * t13433;
    let t46696 = 0.58281247449959539508e2_f64 * t597 * t4614 * t13474;
    let t46699 = 0.23005755572352449806e1_f64 * t42349;
    let t46703 = 0.59584149919750711116e-1_f64 * t42366;
    let t46704 = 0.59584149919750711116e-1_f64 * t42369;
    (t46683, t46688, t46691, t46696, t46699, t46703, t46704)
}
