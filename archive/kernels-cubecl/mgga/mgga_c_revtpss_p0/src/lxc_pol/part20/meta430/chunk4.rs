//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1622/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1622<F: Float>(t12886: F, t3647: F, t1209: F, t13141: F, t17708: F, t12832: F, t12917: F, t11249: F, t3601: F, t13045: F, t3588: F, t1042: F, t1122: F, t12286: F, t1261: F, t12646: F, t1266: F, t12856: F, t12866: F, t12868: F, t12931: F, t12951: F, t17709: F, t17729: F, t17736: F, t247: F, t3604: F, t3618: F, t3626: F, t3630: F, t372: F, t3720: F, t43789: F, t44377: F, t44501: F, t44534: F, t44536: F, t44548: F, t44551: F, t44552: F, t44559: F, t44561: F, t44568: F) -> (F, F) {
    let t44571 = t3647 * t12886;
    let t44578 = t1209 * t13141 * t17708;
    let t44583 = t12832 * t12917;
    let t44585 = t3601 * t11249;
    let t44586 = t13045 * t3588;
    let t44595 = F::cast_from(0.51448821741683684368e-2_f64) * t44534 * t1042 * t44377 * t44536 + F::cast_from(0.34299214494455789577e-2_f64) * t17729 * t3626 * t12286 * t12931 + F::cast_from(0.57165357490759649296e-3_f64) * t44548 + F::cast_from(0.51448821741683684368e-2_f64) * t44551 * t3720 * t44552 * t3604 + F::cast_from(0.34299214494455789578e-2_f64) * t44559 + F::cast_from(0.34299214494455789577e-2_f64) * t44561 * t12868 + F::cast_from(0.17149607247227894789e-2_f64) * t12866 * t372 * t12951 * t3630 - F::cast_from(0.57165357490759649296e-3_f64) * t44568 * t1266 + F::cast_from(0.19055119163586549765e-2_f64) * t44571 - F::cast_from(0.34299214494455789578e-2_f64) * t17736 * t3626 * t12646 * t1122 + F::cast_from(0.51448821741683684368e-2_f64) * t44578 * t3720 * t44501 * t12856 - F::cast_from(0.34299214494455789577e-2_f64) * t44583 + F::cast_from(0.77173232612525526552e-2_f64) * t17709 * t3720 * t44585 * t44586 + F::cast_from(0.85748036236139473944e-2_f64) * t1261 * t247 * t3618 * t43789;
    (t44585, t44595)
}
