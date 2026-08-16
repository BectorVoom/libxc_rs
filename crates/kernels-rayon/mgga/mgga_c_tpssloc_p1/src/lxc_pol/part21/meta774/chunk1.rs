//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2681/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2681(t28: f64, t1081: f64, t12072: f64, t15952: f64, t18196: f64, t19559: f64, t19564: f64, t21: f64, t3231: f64, t3672: f64, t3673: f64, t39436: f64, t5142: f64, t54370: f64, t56252: f64, t584: f64, t5966: f64, t6312: f64, t9: f64, t9212: f64, zeta_threshold: f64) -> f64 {
    let t29 = t28 <= zeta_threshold;
    let t56347 = piecewise3(t29, 0.0_f64, 40.0_f64 / 81.0_f64 * t39436 * t6312 * t3673 + 64.0_f64 / 27.0_f64 * t15952 * t56252 - 8.0_f64 / 27.0_f64 * t19559 * t3231 + 32.0_f64 / 9.0_f64 * t3672 * t9 * t21 - 16.0_f64 / 9.0_f64 * t5142 * t584 + 16.0_f64 / 3.0_f64 * t5142 * t9212 - 8.0_f64 / 27.0_f64 * t12072 * t5966 * t3673 + 8.0_f64 / 9.0_f64 * t3672 * t18196 * t1081 + 4.0_f64 / 9.0_f64 * t19564 * t3231 - t54370);
    t56347
}
