//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3164/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3164(t3617: f64, t372: f64, t6628: f64, t20973: f64, t5391: f64, t5381: f64, t12916: f64, t24735: f64, t5331: f64, t12809: f64, t17351: f64, t17661: f64, t21222: f64, t21246: f64, t21267: f64, t21275: f64, t3611: f64, t3720: f64, t44264: f64, t44510: f64, t5047: f64, t70091: f64, t70102: f64, t70959: f64, t82321: f64) -> (f64, f64) {
    let t83125 = t372 * t3617 * t6628;
    let t83130 = t5391 * t20973;
    let t83136 = t5381 * t20973;
    let t83143 = t5331 * t12916 * t24735;
    let t83145 = -0.45732285992607719436e-2_f64 * t70091 - 0.57165357490759649296e-3_f64 * t70102 + 0.85748036236139473947e-3_f64 * t44510 * t17661 * t21222 + 0.63517063878621832551e-4_f64 * t44264 - 0.7145669686344956162e-3_f64 * t17351 * t83125 * t3611 * t5047 + 0.15244095330869239812e-2_f64 * t83130 + 0.12862205435420921092e-2_f64 * t21275 * t21246 + 0.20579528696673473746e-1_f64 * t70959 * t21267 - 0.28582678745379824648e-3_f64 * t83136 + 0.64311027177104605458e-3_f64 * t12809 * t3720 * t82321 * t3611 - 0.42874018118069736972e-3_f64 * t83143;
    (t83125, t83145)
}
