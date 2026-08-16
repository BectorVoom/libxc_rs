//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2930/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2930<F: Float>(t17297: F, t2929: F, t4497: F, t959: F, t2904: F, t59975: F, t951: F, t18065: F, t225: F, t10165: F, t10170: F, t1052: F, t1066: F, t11010: F, t13939: F, t14658: F, t1625: F, t1634: F, t1635: F, t17583: F, t18062: F, t18166: F, t3026: F, t3169: F, t3174: F, t3175: F, t3206: F, t388: F, t43604: F, t4552: F, t4657: F, t50625: F, t50632: F, t50653: F, t50703: F, t5919: F, t5920: F, t5944: F) -> (F, F, F) {
    let t60963 = t2929 * t17297;
    let t60966 = F::cast_from(0.34631718211362927518e2_f64) * t959 * t60963 * t4497;
    let t60970 = F::cast_from(0.23392894490538584828e1_f64) * t959 * t2904 * t59975 * t951;
    let t60971 = t18065 * t225;
    let t61010 = -F::cast_from(6.0_f64) * t10165 * t1052 * t3206 * t5919 + F::cast_from(4.0_f64) * t1052 * t14658 * t1634 * t3174 + F::cast_from(24.0_f64) * t1052 * t3175 * t43604 * t5919 + F::cast_from(2.0_f64) * t13939 * t1625 * t388 + F::cast_from(4.0_f64) * t388 * t4552 * t4657 + F::cast_from(2.0_f64) * t10170 * t5920 - t10170 * t5944 - F::cast_from(4.0_f64) * t1066 * t60971 - t11010 * t5944 - F::cast_from(2.0_f64) * t1635 * t50625 - F::cast_from(2.0_f64) * t1635 * t50632 - F::cast_from(4.0_f64) * t1635 * t50653 - F::cast_from(2.0_f64) * t1635 * t50703 + F::cast_from(8.0_f64) * t17583 * t3026 + F::cast_from(4.0_f64) * t18062 * t3026 - F::cast_from(2.0_f64) * t18166 * t3169;
    (t60966, t60970, t61010)
}
