//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 951/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk951<F: Float>(t14609: F, t1557: F, t19948: F, t2332: F, t26914: F, t26919: F, t26936: F, t27008: F, t27037: F, t27694: F, t31149: F, t31153: F, t31168: F, t31173: F, t31177: F, t31181: F, t31184: F, t31420: F, t31439: F, t31614: F, t4347: F, t548: F) -> (F,) {
    let t31742 = -0.34822083333333333333e-2 * t31149 - 0.69644166666666666665e-2 * t31153 - 0.77382407407407407405e-3 * t19948 - 0.77382407407407407405e-3 * t26914 + 0.46429444444444444443e-2 * t26919 + 0.34822083333333333333e-2 * t26936 + 0.223494e0 * t4347 * t31420 + 0.69644166666666666665e-2 * t31168 - 0.34822083333333333333e-2 * t31173 - 0.11607361111111111111e-2 * t31177 + 0.51588271604938271604e-3 * t31181 - 0.52233124999999999998e-2 * t31184 - 0.579e0 * t27694 * t2332 - 0.386e0 * t1557 * t31439 - 0.43134342e-1 * t14609 * t31439 + t31614 * t548 + 0.38691203703703703703e-2 * t27008 + 0.30952962962962962963e-2 * t27037;
    (t31742,)
}
