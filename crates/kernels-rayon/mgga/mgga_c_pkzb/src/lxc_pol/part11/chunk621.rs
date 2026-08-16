//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 621/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk621(t164: f64, t3441: f64, t51: f64, t592: f64, t3411: f64, t3401: f64, t616: f64, t1774: f64, t3396: f64, t615: f64, t1701: f64, t1706: f64, t1718: f64, t1733: f64, t1768: f64, t2580: f64, t2598: f64, t2658: f64, t3403: f64, t3407: f64, t3413: f64, t3418: f64, t580: f64, t590: f64, t612: f64) -> (f64, f64, f64, f64, f64) {
    let t3444 = t592 * t51 * t3441 * t164;
    let t3448 = t592 * t3411 * t164;
    let t3452 = t616 * t3401;
    let t3453 = t1774 * t3452;
    let t3456 = t616 * t3396;
    let t3457 = t615 * t3456;
    let t3460 = t1701 + 7.0_f64 / 72.0_f64 * t2580 + t1706 * t3403 / 16.0_f64 - t580 * t3407 / 48.0_f64 + 0.42874018118069736972e-3_f64 * t1718 * t3413 + 0.20007875121765877254e-2_f64 * t2598 + 0.17149607247227894789e-2_f64 * t1733 * t3418 - 0.21437009059034868486e-3_f64 * t590 * t3444 - 0.21437009059034868486e-3_f64 * t590 * t3448 + t1768 + 0.80031500487063509015e-2_f64 * t2658 + 0.42874018118069736972e-2_f64 * t612 * t3453 - 0.85748036236139473944e-3_f64 * t612 * t3457;
    (t3444, t3448, t3453, t3457, t3460)
}
