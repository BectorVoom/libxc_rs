//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1012/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1012<F: Float>(t401: F, t4491: F, t428: F, t1526: F, t4422: F, t7705: F, t11262: F, t19950: F, t11280: F, t1527: F, t1528: F, t15567: F, t15568: F, t15596: F, t15599: F, t15625: F, t15737: F, t15742: F, t15746: F, t15902: F, t15917: F, t15927: F, t15936: F, t15940: F, t15968: F, t2984: F, t3088: F, t38310: F, t61123: F, t61130: F, t61132: F, t61163: F, t8633: F, t942: F) -> (F, F, F) {
    let t73881 = t4491 * t401;
    let t74254 = t4491 * t428;
    let t75878 = t1526 * t7705 * t4422;
    let t75881 = t1526 * t11262 * t19950;
    let t75912 = t15599 + t61130 - t61132 - 2.0 / 9.0 * t15567 * t8633 * t942 * t2984 - t75878 / 18.0 - t75881 / 27.0 - t1526 * t1527 * t15927 / 12.0 - t1526 * t1527 * t1528 * t15625 / 12.0 - 4.0 / 9.0 * t61123 * t15568 * t15746 + 2.0 / 3.0 * t15567 * t15568 * t15742 - 7.0 / 27.0 * t15567 * t61163 * t15737 + t38310 / 54.0 + t15596 + t15968 - t1526 * t1527 * t15902 / 12.0 - t1526 * t11280 * t15936 / 3.0 + t1526 * t1527 * t15917 / 6.0 - t1526 * t3088 * t15940 / 9.0;
    (t73881, t74254, t75912)
}
