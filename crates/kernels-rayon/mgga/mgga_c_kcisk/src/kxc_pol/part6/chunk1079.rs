//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 1079/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk1079(t14609: f64, t1557: f64, t19948: f64, t2332: f64, t26914: f64, t26919: f64, t26936: f64, t27008: f64, t27037: f64, t27694: f64, t31149: f64, t31153: f64, t31168: f64, t31173: f64, t31177: f64, t31181: f64, t31184: f64, t31420: f64, t31439: f64, t31614: f64, t4347: f64, t548: f64) -> f64 {
    let t31742 = -0.34822083333333333333e-2_f64 * t31149 - 0.69644166666666666665e-2_f64 * t31153 - 0.77382407407407407405e-3_f64 * t19948 - 0.77382407407407407405e-3_f64 * t26914 + 0.46429444444444444443e-2_f64 * t26919 + 0.34822083333333333333e-2_f64 * t26936 + 0.223494e0_f64 * t4347 * t31420 + 0.69644166666666666665e-2_f64 * t31168 - 0.34822083333333333333e-2_f64 * t31173 - 0.11607361111111111111e-2_f64 * t31177 + 0.51588271604938271604e-3_f64 * t31181 - 0.52233124999999999998e-2_f64 * t31184 - 0.579e0_f64 * t27694 * t2332 - 0.386e0_f64 * t1557 * t31439 - 0.43134342e-1_f64 * t14609 * t31439 + t31614 * t548 + 0.38691203703703703703e-2_f64 * t27008 + 0.30952962962962962963e-2_f64 * t27037;
    t31742
}
