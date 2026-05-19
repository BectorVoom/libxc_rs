//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1216/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1216<F: Float>(t10819: F, t6737: F, t1267: F, t3622: F, t6842: F, t11183: F, t1240: F, t13382: F, t13399: F, t15134: F, t15602: F, t15607: F, t19704: F, t19708: F, t19713: F, t19717: F, t19720: F, t19725: F, t19729: F, t19738: F, t3644: F, t5282: F, t5345: F) -> F {
    let t20314 = t6737 * t10819;
    let t20315 = t20314 * t1267;
    let t20330 = t6842 * t3622;
    let t20331 = t20330 * t1267;
    let t20338 = -F::cast_from(0.15476481481481481481e-2_f64) * t19704 + F::cast_from(0.77382407407407407407e-3_f64) * t19708 + F::cast_from(0.25794135802469135802e-2_f64) * t19713 + F::cast_from(0.46429444444444444444e-2_f64) * t19717 - F::new(0.13345e0) * t1240 * t20315 - F::cast_from(0.46429444444444444443e-2_f64) * t19720 - F::cast_from(0.41270617283950617283e-2_f64) * t13382 - F::cast_from(0.51588271604938271603e-3_f64) * t19725 - F::cast_from(0.30952962962962962962e-2_f64) * t19729 - F::cast_from(0.178244852896875e-2_f64) * t11183 * t20315 - F::cast_from(0.2671335375e-1_f64) * t3644 * t20315 + F::new(0.13345e0) * t5345 * t5282 + F::cast_from(0.178089025e-1_f64) * t15134 * t5282 + F::new(0.66725e-1) * t1240 * t20331 + F::cast_from(0.890445125e-2_f64) * t3644 * t20331 + t15602 + F::cast_from(0.30952962962962962962e-2_f64) * t13399 + t15607 - F::cast_from(0.11607361111111111111e-2_f64) * t19738;
    t20338
}
