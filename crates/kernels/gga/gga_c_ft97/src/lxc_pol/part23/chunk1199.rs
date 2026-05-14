//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1199/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1199<F: Float>(t1882: F, t31135: F, t31252: F, t31258: F, t31249: F, t10051: F, t110364: F, t110380: F, t18211: F, t18391: F, t1901: F, t24412: F, t2574: F, t2594: F, t28023: F, t28129: F, t28136: F, t28299: F, t31060: F, t3837: F, t3859: F, t3864: F, t446: F, t4965: F, t6166: F, t6194: F, t65408: F, t67847: F, t6940: F, t6947: F, t713: F, t729: F, t762: F) -> (F,) {
    let t122281 = t1882 * t31135;
    let t122283 = t1882 * t31252;
    let t122297 = t1882 * t31258;
    let t122303 = t1882 * t31249;
    let t122321 = t446 * t729 * t18391 * t6166 / 3.0 + 2.0 / 3.0 * t122281 - 4.0 / 9.0 * t122283 + 4.0 / 3.0 * t446 * t2574 * t6947 * t3837 - 2.0 / 3.0 * t446 * t729 * t24412 * t18211 - 2.0 / 27.0 * t446 * t2594 * t6194 * t4965 + t122297 / 9.0 + 2.0 / 3.0 * t446 * t729 * t28023 * t3859 - 4.0 / 9.0 * t122303 - 4.0 / 3.0 * t1901 * t65408 * t28136 - 4.0 / 3.0 * t1901 * t67847 * t28129 - t110364 - 4.0 * t1901 * t28299 * t10051 * t6940 * t3864 + t446 * t729 * t762 * t31060 * t713 / 3.0 + t110380;
    (t122321,)
}
