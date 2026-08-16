//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 1021/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk1021(t15299: f64, t18997: f64, t5424: f64, t684: f64, t835: f64, t4176: f64, t4246: f64, t840: f64, t10732: f64, t10735: f64, t15318: f64, t15329: f64, t15334: f64, t15336: f64, t15376: f64, t15382: f64, t15384: f64, t15400: f64, t15419: f64, t15420: f64, t1901: f64, t446: f64) -> f64 {
    let t19793 = t15299 * t18997;
    let t19799 = t835 * t5424 * t684;
    let t19803 = t840 * t4246 * t4176;
    let t19809 = -4.0_f64 / 9.0_f64 * t1901 * t19793 - 8.0_f64 / 81.0_f64 * t15318 + 8.0_f64 / 27.0_f64 * t15329 + t15334 + t15336 - t15376 - t15382 - t15384 + t15400 - t446 * t19799 / 9.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t19803 - 4.0_f64 / 81.0_f64 * t10732 + t15419 - 8.0_f64 / 27.0_f64 * t15420 - 4.0_f64 / 27.0_f64 * t10735;
    t19809
}
