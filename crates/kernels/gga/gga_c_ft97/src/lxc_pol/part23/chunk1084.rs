//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1084/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1084<F: Float>(t5009: F, t79931: F, t5014: F, t679: F, t689: F, t5049: F, t17988: F, t38176: F, t13598: F, t1526: F, t21103: F, t4922: F, t9483: F, t10915: F, t1131: F, t15567: F, t17687: F, t17694: F, t17727: F, t17732: F, t17744: F, t17749: F, t17753: F, t17761: F, t17766: F, t17771: F, t17780: F, t18139: F, t231: F, t2320: F, t2917: F, t342: F, t343: F, t3691: F, t3700: F, t3806: F, t61123: F, t69066: F, t69068: F, t69073: F, t69081: F, t69108: F) -> (F, F, F, F) {
    let t79932 = t79931 * t5009;
    let t79933 = t5014 * t679;
    let t79935 = t79932 * t79933 * t689;
    let t79942 = t5049 * t679 * t689;
    let t80170 = t38176 * t17988;
    let t81955 = t1526 * t13598 * t21103;
    let t81958 = t1526 * t9483 * t4922;
    let t81963 = -7.0 / 27.0 * t15567 * t69108 * t17749 + 4.0 / 9.0 * t61123 * t17687 * t17753 - t15567 * t17694 * t17744 / 2.0 - 2.0 / 3.0 * t61123 * t17694 * t17780 + t15567 * t17694 * t17727 / 6.0 - t15567 * t17687 * t17732 / 9.0 - t342 * t343 * t231 * t18139 / 4.0 + t15567 * t2917 * t1131 * t3700 / 3.0 - 2.0 / 9.0 * t15567 * t10915 * t1131 * t3691 + 2.0 / 3.0 * t15567 * t17687 * t17766 + t69066 - t69068 - t1526 * t2320 * t17771 / 12.0 + t69073 / 9.0 - t69081 - t81955 / 27.0 - t81958 / 18.0 - t1526 * t3806 * t17761 / 9.0;
    (t79935, t79942, t80170, t81963)
}
