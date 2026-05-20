//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3759/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3759<F: Float>(t12855: F, t12916: F, t20977: F, t12787: F, t12866: F, t17661: F, t17678: F, t17683: F, t17689: F, t17693: F, t17729: F, t17730: F, t17736: F, t17799: F, t20921: F, t20923: F, t21046: F, t3362: F, t3625: F, t3626: F, t3629: F, t4181: F, t44510: F, t44517: F, t5245: F, t56861: F, t57621: F, t58960: F, t59017: F, t59220: F, t6421: F, t70910: F, t70944: F, t71314: F, t71452: F) -> F {
    let t71630 = t12855 * t12916 * t20977;
    let t71667 = F::cast_from(0.42874018118069736972e-3_f64) * t59017 * t21046 - F::cast_from(0.57165357490759649296e-3_f64) * t71630 - F::cast_from(0.95275595817932748826e-3_f64) * t56861 * t20923 - F::cast_from(0.95275595817932748826e-3_f64) * t17729 * t12787 * t5245 * t3362 * t4181 + F::cast_from(0.57165357490759649296e-3_f64) * t44510 * t17661 * t17678 - F::cast_from(0.28582678745379824648e-3_f64) * t44517 * t17661 * t17683 - F::cast_from(0.11433071498151929859e-2_f64) * t17693 * t17799 * t70910 - F::cast_from(0.17149607247227894789e-2_f64) * t17693 * t57621 * t71314 - F::cast_from(0.47637797908966374413e-3_f64) * t12866 * t58960 * t17689 + F::cast_from(0.6351706387862183255e-3_f64) * t59220 + F::cast_from(0.17149607247227894789e-2_f64) * t17729 * t3626 * t6421 * t17730 + F::cast_from(0.19055119163586549765e-2_f64) * t17736 * t12787 * t20921 * t71452 - F::cast_from(0.28582678745379824648e-3_f64) * t3625 * t3626 * t70944 * t3629;
    t71667
}
