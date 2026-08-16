//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 886/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk886(t2105: f64, t9571: f64, t2739: f64, t287: f64, t1137: f64, t154: f64, t3542: f64, t5663: f64, t276: f64, t2104: f64, t2887: f64, t2899: f64, t2922: f64, t3631: f64, t5691: f64, t735: f64, t757: f64, t7718: f64, t7756: f64, t7760: f64, t7767: f64, t9542: f64, t9547: f64, t9550: f64, t9555: f64, t9559: f64, t9564: f64, t9568: f64) -> (f64, f64, f64, f64, f64) {
    let t9572 = t2105 * t9571;
    let t9575 = t287 * t2739;
    let t9576 = t1137 * t9575;
    let t9577 = t2105 * t9576;
    let t9583 = t154 * t5663 * t3542;
    let t9584 = t276 * t9583;
    let t9586 = -t7718 + t5691 / 432.0_f64 + 0.19055119163586549765e-3_f64 * t7756 + 0.30488190661738479625e-2_f64 * t7760 - t7767 + 0.21437009059034868486e-3_f64 * t757 * t9542 - t9547 / 288.0_f64 - t276 * t9550 / 96.0_f64 + t2887 * t9555 / 48.0_f64 - 0.42874018118069736972e-3_f64 * t2104 * t9559 + 0.42874018118069736972e-3_f64 * t2899 * t9564 - 0.21437009059034868486e-3_f64 * t2922 * t9568 - 0.85748036236139473944e-3_f64 * t2104 * t9572 - 0.85748036236139473944e-3_f64 * t2104 * t9577 - t735 * t3631 / 18.0_f64 + t9584 / 144.0_f64;
    (t9575, t9576, t9583, t9584, t9586)
}
