//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1628/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1628(t1122: f64, t1261: f64, t247: f64, t44701: f64, t12776: f64, t12788: f64, t12812: f64, t12835: f64, t12866: f64, t12867: f64, t12936: f64, t3613: f64, t3718: f64, t372: f64, t3720: f64, t44431: f64, t44658: f64, t44661: f64, t44664: f64, t44672: f64, t44675: f64, t44678: f64, t44681: f64, t44696: f64, t44698: f64, t5352: f64) -> f64 {
    let t44704 = t1261 * t247 * t44701 * t1122;
    let t44706 = 0.34299214494455789578e-2_f64 * t44658 + 0.34299214494455789578e-2_f64 * t44661 + 0.25724410870841842184e-2_f64 * t44664 * t12812 - 0.85748036236139473944e-3_f64 * t3718 * t3720 * t44431 * t5352 - 0.17149607247227894789e-2_f64 * t44672 - 0.57165357490759649296e-3_f64 * t44675 - 0.22866142996303859719e-2_f64 * t44678 + 0.22866142996303859718e-2_f64 * t44681 - 0.28582678745379824648e-2_f64 * t12866 * t372 * t12936 * t12788 + 0.17149607247227894789e-2_f64 * t12866 * t12867 * t12776 + 0.17149607247227894789e-2_f64 * t12866 * t12867 * t12835 - 0.31758531939310916276e-3_f64 * t44696 - 0.12862205435420921092e-2_f64 * t44698 * t3613 - 0.16937883700965822013e-3_f64 * t44704;
    t44706
}
