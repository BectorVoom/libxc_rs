//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2930/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2930(t17297: f64, t2929: f64, t4497: f64, t959: f64, t2904: f64, t59975: f64, t951: f64, t18065: f64, t225: f64, t10165: f64, t10170: f64, t1052: f64, t1066: f64, t11010: f64, t13939: f64, t14658: f64, t1625: f64, t1634: f64, t1635: f64, t17583: f64, t18062: f64, t18166: f64, t3026: f64, t3169: f64, t3174: f64, t3175: f64, t3206: f64, t388: f64, t43604: f64, t4552: f64, t4657: f64, t50625: f64, t50632: f64, t50653: f64, t50703: f64, t5919: f64, t5920: f64, t5944: f64) -> (f64, f64, f64) {
    let t60963 = t2929 * t17297;
    let t60966 = 0.34631718211362927518e2_f64 * t959 * t60963 * t4497;
    let t60970 = 0.23392894490538584828e1_f64 * t959 * t2904 * t59975 * t951;
    let t60971 = t18065 * t225;
    let t61010 = -6.0_f64 * t10165 * t1052 * t3206 * t5919 + 4.0_f64 * t1052 * t14658 * t1634 * t3174 + 24.0_f64 * t1052 * t3175 * t43604 * t5919 + 2.0_f64 * t13939 * t1625 * t388 + 4.0_f64 * t388 * t4552 * t4657 + 2.0_f64 * t10170 * t5920 - t10170 * t5944 - 4.0_f64 * t1066 * t60971 - t11010 * t5944 - 2.0_f64 * t1635 * t50625 - 2.0_f64 * t1635 * t50632 - 4.0_f64 * t1635 * t50653 - 2.0_f64 * t1635 * t50703 + 8.0_f64 * t17583 * t3026 + 4.0_f64 * t18062 * t3026 - 2.0_f64 * t18166 * t3169;
    (t60966, t60970, t61010)
}
