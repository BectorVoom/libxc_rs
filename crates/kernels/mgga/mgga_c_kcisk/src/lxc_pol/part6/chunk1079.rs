//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 1079/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk1079<F: Float>(t14609: F, t1557: F, t19948: F, t2332: F, t26914: F, t26919: F, t26936: F, t27008: F, t27037: F, t27694: F, t31149: F, t31153: F, t31168: F, t31173: F, t31177: F, t31181: F, t31184: F, t31420: F, t31439: F, t31614: F, t4347: F, t548: F) -> F {
    let t31742 = -F::cast_from(0.34822083333333333333e-2_f64) * t31149 - F::cast_from(0.69644166666666666665e-2_f64) * t31153 - F::cast_from(0.77382407407407407405e-3_f64) * t19948 - F::cast_from(0.77382407407407407405e-3_f64) * t26914 + F::cast_from(0.46429444444444444443e-2_f64) * t26919 + F::cast_from(0.34822083333333333333e-2_f64) * t26936 + F::new(0.223494e0) * t4347 * t31420 + F::cast_from(0.69644166666666666665e-2_f64) * t31168 - F::cast_from(0.34822083333333333333e-2_f64) * t31173 - F::cast_from(0.11607361111111111111e-2_f64) * t31177 + F::cast_from(0.51588271604938271604e-3_f64) * t31181 - F::cast_from(0.52233124999999999998e-2_f64) * t31184 - F::new(0.579e0) * t27694 * t2332 - F::new(0.386e0) * t1557 * t31439 - F::new(0.43134342e-1) * t14609 * t31439 + t31614 * t548 + F::cast_from(0.38691203703703703703e-2_f64) * t27008 + F::cast_from(0.30952962962962962963e-2_f64) * t27037;
    t31742
}
