//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3759/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3759(t12855: f64, t12916: f64, t20977: f64, t12787: f64, t12866: f64, t17661: f64, t17678: f64, t17683: f64, t17689: f64, t17693: f64, t17729: f64, t17730: f64, t17736: f64, t17799: f64, t20921: f64, t20923: f64, t21046: f64, t3362: f64, t3625: f64, t3626: f64, t3629: f64, t4181: f64, t44510: f64, t44517: f64, t5245: f64, t56861: f64, t57621: f64, t58960: f64, t59017: f64, t59220: f64, t6421: f64, t70910: f64, t70944: f64, t71314: f64, t71452: f64) -> f64 {
    let t71630 = t12855 * t12916 * t20977;
    let t71667 = 0.42874018118069736972e-3_f64 * t59017 * t21046 - 0.57165357490759649296e-3_f64 * t71630 - 0.95275595817932748826e-3_f64 * t56861 * t20923 - 0.95275595817932748826e-3_f64 * t17729 * t12787 * t5245 * t3362 * t4181 + 0.57165357490759649296e-3_f64 * t44510 * t17661 * t17678 - 0.28582678745379824648e-3_f64 * t44517 * t17661 * t17683 - 0.11433071498151929859e-2_f64 * t17693 * t17799 * t70910 - 0.17149607247227894789e-2_f64 * t17693 * t57621 * t71314 - 0.47637797908966374413e-3_f64 * t12866 * t58960 * t17689 + 0.6351706387862183255e-3_f64 * t59220 + 0.17149607247227894789e-2_f64 * t17729 * t3626 * t6421 * t17730 + 0.19055119163586549765e-2_f64 * t17736 * t12787 * t20921 * t71452 - 0.28582678745379824648e-3_f64 * t3625 * t3626 * t70944 * t3629;
    t71667
}
