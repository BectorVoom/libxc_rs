//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2960/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2960<F: Float>(t12160: F, t15688: F, t1011: F, t15689: F, t15692: F, t15693: F, t15906: F, t15907: F, t16012: F, t16081: F, t16098: F, t3117: F, t42546: F, t4915: F, t4919: F, t51869: F, t51905: F, t51998: F, t53545: F, t53885: F, t53898: F, t53901: F, t53904: F, t53909: F) -> F {
    let t53914 = t12160 * t15688;
    let t53920 = -F::cast_from(0.91464571985215438873e-2_f64) * t53885 * t16098 - t1011 * t4915 * t51869 / F::new(144.0) - F::new(7.0) / F::new(54.0) * t1011 * t16012 * t51998 + t1011 * t4919 * t51905 / F::new(72.0) - F::cast_from(0.42874018118069736972e-3_f64) * t53898 - F::cast_from(0.95275595817932748826e-4_f64) * t53901 + F::cast_from(0.28582678745379824648e-3_f64) * t42546 + F::cast_from(0.38586616306262763276e-2_f64) * t16081 * t3117 * t15907 * t53904 - F::cast_from(0.38586616306262763275e-2_f64) * t15906 * t3117 * t15907 * t53909 - F::cast_from(0.85748036236139473944e-3_f64) * t53914 * t15693 - F::cast_from(0.85748036236139473944e-3_f64) * t15689 * t53545 * t15692;
    t53920
}
