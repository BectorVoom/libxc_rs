//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 621/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk621<F: Float>(t164: F, t3441: F, t51: F, t592: F, t3411: F, t3401: F, t616: F, t1774: F, t3396: F, t615: F, t1701: F, t1706: F, t1718: F, t1733: F, t1768: F, t2580: F, t2598: F, t2658: F, t3403: F, t3407: F, t3413: F, t3418: F, t580: F, t590: F, t612: F) -> (F, F, F, F, F) {
    let t3444 = t592 * t51 * t3441 * t164;
    let t3448 = t592 * t3411 * t164;
    let t3452 = t616 * t3401;
    let t3453 = t1774 * t3452;
    let t3456 = t616 * t3396;
    let t3457 = t615 * t3456;
    let t3460 = t1701 + F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t2580 + t1706 * t3403 / F::cast_from(16.0_f64) - t580 * t3407 / F::cast_from(48.0_f64) + F::cast_from(0.42874018118069736972e-3_f64) * t1718 * t3413 + F::cast_from(0.20007875121765877254e-2_f64) * t2598 + F::cast_from(0.17149607247227894789e-2_f64) * t1733 * t3418 - F::cast_from(0.21437009059034868486e-3_f64) * t590 * t3444 - F::cast_from(0.21437009059034868486e-3_f64) * t590 * t3448 + t1768 + F::cast_from(0.80031500487063509015e-2_f64) * t2658 + F::cast_from(0.42874018118069736972e-2_f64) * t612 * t3453 - F::cast_from(0.85748036236139473944e-3_f64) * t612 * t3457;
    (t3444, t3448, t3453, t3457, t3460)
}
