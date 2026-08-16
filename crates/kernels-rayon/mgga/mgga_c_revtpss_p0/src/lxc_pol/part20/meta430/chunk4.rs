//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1622/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1622(t12886: f64, t3647: f64, t1209: f64, t13141: f64, t17708: f64, t12832: f64, t12917: f64, t11249: f64, t3601: f64, t13045: f64, t3588: f64, t1042: f64, t1122: f64, t12286: f64, t1261: f64, t12646: f64, t1266: f64, t12856: f64, t12866: f64, t12868: f64, t12931: f64, t12951: f64, t17709: f64, t17729: f64, t17736: f64, t247: f64, t3604: f64, t3618: f64, t3626: f64, t3630: f64, t372: f64, t3720: f64, t43789: f64, t44377: f64, t44501: f64, t44534: f64, t44536: f64, t44548: f64, t44551: f64, t44552: f64, t44559: f64, t44561: f64, t44568: f64) -> (f64, f64) {
    let t44571 = t3647 * t12886;
    let t44578 = t1209 * t13141 * t17708;
    let t44583 = t12832 * t12917;
    let t44585 = t3601 * t11249;
    let t44586 = t13045 * t3588;
    let t44595 = 0.51448821741683684368e-2_f64 * t44534 * t1042 * t44377 * t44536 + 0.34299214494455789577e-2_f64 * t17729 * t3626 * t12286 * t12931 + 0.57165357490759649296e-3_f64 * t44548 + 0.51448821741683684368e-2_f64 * t44551 * t3720 * t44552 * t3604 + 0.34299214494455789578e-2_f64 * t44559 + 0.34299214494455789577e-2_f64 * t44561 * t12868 + 0.17149607247227894789e-2_f64 * t12866 * t372 * t12951 * t3630 - 0.57165357490759649296e-3_f64 * t44568 * t1266 + 0.19055119163586549765e-2_f64 * t44571 - 0.34299214494455789578e-2_f64 * t17736 * t3626 * t12646 * t1122 + 0.51448821741683684368e-2_f64 * t44578 * t3720 * t44501 * t12856 - 0.34299214494455789577e-2_f64 * t44583 + 0.77173232612525526552e-2_f64 * t17709 * t3720 * t44585 * t44586 + 0.85748036236139473944e-2_f64 * t1261 * t247 * t3618 * t43789;
    (t44585, t44595)
}
