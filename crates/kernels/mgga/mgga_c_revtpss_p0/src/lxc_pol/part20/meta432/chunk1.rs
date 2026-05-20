//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1628/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1628<F: Float>(t1122: F, t1261: F, t247: F, t44701: F, t12776: F, t12788: F, t12812: F, t12835: F, t12866: F, t12867: F, t12936: F, t3613: F, t3718: F, t372: F, t3720: F, t44431: F, t44658: F, t44661: F, t44664: F, t44672: F, t44675: F, t44678: F, t44681: F, t44696: F, t44698: F, t5352: F) -> F {
    let t44704 = t1261 * t247 * t44701 * t1122;
    let t44706 = F::cast_from(0.34299214494455789578e-2_f64) * t44658 + F::cast_from(0.34299214494455789578e-2_f64) * t44661 + F::cast_from(0.25724410870841842184e-2_f64) * t44664 * t12812 - F::cast_from(0.85748036236139473944e-3_f64) * t3718 * t3720 * t44431 * t5352 - F::cast_from(0.17149607247227894789e-2_f64) * t44672 - F::cast_from(0.57165357490759649296e-3_f64) * t44675 - F::cast_from(0.22866142996303859719e-2_f64) * t44678 + F::cast_from(0.22866142996303859718e-2_f64) * t44681 - F::cast_from(0.28582678745379824648e-2_f64) * t12866 * t372 * t12936 * t12788 + F::cast_from(0.17149607247227894789e-2_f64) * t12866 * t12867 * t12776 + F::cast_from(0.17149607247227894789e-2_f64) * t12866 * t12867 * t12835 - F::cast_from(0.31758531939310916276e-3_f64) * t44696 - F::cast_from(0.12862205435420921092e-2_f64) * t44698 * t3613 - F::cast_from(0.16937883700965822013e-3_f64) * t44704;
    t44706
}
