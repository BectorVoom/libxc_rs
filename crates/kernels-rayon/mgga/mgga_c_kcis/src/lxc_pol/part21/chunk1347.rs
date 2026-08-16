//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1347/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1347(t10819: f64, t330: f64, t1646: f64, t3620: f64, t5310: f64, t26966: f64, t28093: f64, t26960: f64, t27014: f64, t27042: f64, t28107: f64, t28113: f64, t28132: f64, t28153: f64, t92718: f64, t92730: f64, t92740: f64, t93023: f64, t95751: f64, t95756: f64, t95759: f64) -> (f64, f64) {
    let t96854 = t10819 * t330;
    let t96857 = t5310 * t96854 * t1646 * t3620;
    let t96868 = 0.61782407407407407408e-3_f64 * t26966 * t28093;
    let t96869 = 0.77382407407407407407e-3_f64 * t95751 + 0.30952962962962962962e-2_f64 * t95756 - 0.46429444444444444444e-2_f64 * t95759 + 0.69505208333333333334e-3_f64 * t27014 * t28153 + 0.24734586805555555556e-3_f64 * t27042 * t28132 - 0.23168402777777777778e-3_f64 * t26960 * t96857 + 0.23168402777777777778e-3_f64 * t93023 * t28107 + 0.7722800925925925926e-4_f64 * t92718 + 0.23168402777777777778e-3_f64 * t93023 * t28113 - 0.51588271604938271604e-3_f64 * t92730 + 0.15476481481481481481e-2_f64 * t92740 - t96868;
    (t96857, t96869)
}
