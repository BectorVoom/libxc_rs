//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1216/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1216(t10819: f64, t6737: f64, t1267: f64, t3622: f64, t6842: f64, t11183: f64, t1240: f64, t13382: f64, t13399: f64, t15134: f64, t15602: f64, t15607: f64, t19704: f64, t19708: f64, t19713: f64, t19717: f64, t19720: f64, t19725: f64, t19729: f64, t19738: f64, t3644: f64, t5282: f64, t5345: f64) -> f64 {
    let t20314 = t6737 * t10819;
    let t20315 = t20314 * t1267;
    let t20330 = t6842 * t3622;
    let t20331 = t20330 * t1267;
    let t20338 = -0.15476481481481481481e-2_f64 * t19704 + 0.77382407407407407407e-3_f64 * t19708 + 0.25794135802469135802e-2_f64 * t19713 + 0.46429444444444444444e-2_f64 * t19717 - 0.13345e0_f64 * t1240 * t20315 - 0.46429444444444444443e-2_f64 * t19720 - 0.41270617283950617283e-2_f64 * t13382 - 0.51588271604938271603e-3_f64 * t19725 - 0.30952962962962962962e-2_f64 * t19729 - 0.178244852896875e-2_f64 * t11183 * t20315 - 0.2671335375e-1_f64 * t3644 * t20315 + 0.13345e0_f64 * t5345 * t5282 + 0.178089025e-1_f64 * t15134 * t5282 + 0.66725e-1_f64 * t1240 * t20331 + 0.890445125e-2_f64 * t3644 * t20331 + t15602 + 0.30952962962962962962e-2_f64 * t13399 + t15607 - 0.11607361111111111111e-2_f64 * t19738;
    t20338
}
