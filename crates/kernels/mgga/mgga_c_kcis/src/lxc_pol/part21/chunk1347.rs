//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1347/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1347<F: Float>(t10819: F, t330: F, t1646: F, t3620: F, t5310: F, t26966: F, t28093: F, t26960: F, t27014: F, t27042: F, t28107: F, t28113: F, t28132: F, t28153: F, t92718: F, t92730: F, t92740: F, t93023: F, t95751: F, t95756: F, t95759: F) -> (F, F) {
    let t96854 = t10819 * t330;
    let t96857 = t5310 * t96854 * t1646 * t3620;
    let t96868 = F::cast_from(0.61782407407407407408e-3_f64) * t26966 * t28093;
    let t96869 = F::cast_from(0.77382407407407407407e-3_f64) * t95751 + F::cast_from(0.30952962962962962962e-2_f64) * t95756 - F::cast_from(0.46429444444444444444e-2_f64) * t95759 + F::cast_from(0.69505208333333333334e-3_f64) * t27014 * t28153 + F::cast_from(0.24734586805555555556e-3_f64) * t27042 * t28132 - F::cast_from(0.23168402777777777778e-3_f64) * t26960 * t96857 + F::cast_from(0.23168402777777777778e-3_f64) * t93023 * t28107 + F::cast_from(0.7722800925925925926e-4_f64) * t92718 + F::cast_from(0.23168402777777777778e-3_f64) * t93023 * t28113 - F::cast_from(0.51588271604938271604e-3_f64) * t92730 + F::cast_from(0.15476481481481481481e-2_f64) * t92740 - t96868;
    (t96857, t96869)
}
