//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1013/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1013<F: Float>(t19965: F, t342: F, t630: F, t1526: F, t19961: F, t7705: F, t19957: F, t1527: F, t15567: F, t15568: F, t15575: F, t15594: F, t15752: F, t15756: F, t15763: F, t15768: F, t15885: F, t15932: F, t2258: F, t2993: F, t343: F, t38355: F, t38369: F, t61123: F, t61147: F, t61174: F, t61180: F, t61184: F, t61197: F, t61199: F, t72: F, t942: F) -> (F,) {
    let t75935 = t342 * t630 * t19965;
    let t75944 = t1526 * t7705 * t19961;
    let t75947 = t1526 * t7705 * t19957;
    let t75949 = -t1526 * t1527 * t15932 / 6.0 - t61147 - t38355 + t38369 / 18.0 - t61174 + t61180 / 9.0 + t15567 * t15575 * t15768 / 6.0 - t15567 * t15568 * t15763 / 9.0 - t15567 * t15575 * t15752 / 2.0 + 2.0 / 3.0 * t61123 * t15575 * t15756 + t15567 * t2258 * t942 * t2993 / 3.0 - t75935 / 12.0 - t342 * t343 * t72 * t15885 / 4.0 + t61184 / 27.0 - t61197 - t61199 / 9.0 + t15594 - t75944 / 36.0 + t75947 / 18.0;
    (t75949,)
}
