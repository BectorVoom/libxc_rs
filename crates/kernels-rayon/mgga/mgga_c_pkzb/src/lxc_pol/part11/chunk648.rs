//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 648/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk648(t301: f64, t3650: f64, t761: f64, t758: f64, t2039: f64, t3639: f64, t179: f64, t2004: f64, t3542: f64, t3515: f64, t780: f64, t2026: f64, t2038: f64, t2047: f64, t2067: f64, t2104: f64, t276: f64, t2884: f64, t2909: f64, t2940: f64, t299: f64, t3631: f64, t3635: f64, t3641: f64, t3647: f64, t757: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3651 = t301 * t3650;
    let t3652 = t3651 * t761;
    let t3653 = t758 * t3652;
    let t3656 = t3639 * t2039;
    let t3657 = t758 * t3656;
    let t3662 = t179 * t2004 * t3542;
    let t3666 = t179 * t780 * t3515;
    let t3669 = -t2047 - t2884 / 144.0_f64 + t276 * t3631 / 48.0_f64 - t276 * t3635 / 96.0_f64 + 0.42874018118069736972e-3_f64 * t2026 * t3641 + 0.28582678745379824648e-3_f64 * t2909 - 0.85748036236139473944e-3_f64 * t2104 * t3647 + 0.21437009059034868486e-3_f64 * t757 * t3653 - 0.21437009059034868486e-3_f64 * t2038 * t3657 - t2067 - 0.57165357490759649296e-3_f64 * t2940 + 0.12862205435420921092e-2_f64 * t299 * t3662 - 0.42874018118069736972e-3_f64 * t299 * t3666;
    (t3651, t3652, t3653, t3656, t3657, t3662, t3666, t3669)
}
