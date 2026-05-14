//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1088/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1088<F: Float>(t10819: F, t6737: F, t1267: F, t3622: F, t6842: F, t11183: F, t1240: F, t13382: F, t13399: F, t15134: F, t15602: F, t15607: F, t19704: F, t19708: F, t19713: F, t19717: F, t19720: F, t19725: F, t19729: F, t19738: F, t3644: F, t5282: F, t5345: F) -> (F,) {
    let t20314 = t6737 * t10819;
    let t20315 = t20314 * t1267;
    let t20330 = t6842 * t3622;
    let t20331 = t20330 * t1267;
    let t20338 = -0.15476481481481481481e-2 * t19704 + 0.77382407407407407407e-3 * t19708 + 0.25794135802469135802e-2 * t19713 + 0.46429444444444444444e-2 * t19717 - 0.13345e0 * t1240 * t20315 - 0.46429444444444444443e-2 * t19720 - 0.41270617283950617283e-2 * t13382 - 0.51588271604938271603e-3 * t19725 - 0.30952962962962962962e-2 * t19729 - 0.178244852896875e-2 * t11183 * t20315 - 0.2671335375e-1 * t3644 * t20315 + 0.13345e0 * t5345 * t5282 + 0.178089025e-1 * t15134 * t5282 + 0.66725e-1 * t1240 * t20331 + 0.890445125e-2 * t3644 * t20331 + t15602 + 0.30952962962962962962e-2 * t13399 + t15607 - 0.11607361111111111111e-2 * t19738;
    (t20338,)
}
