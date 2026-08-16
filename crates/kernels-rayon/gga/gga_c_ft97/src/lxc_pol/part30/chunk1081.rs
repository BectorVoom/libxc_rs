//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1081/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1081(t2567: f64, t7440: f64, t1882: f64, t35699: f64, t35734: f64, t35684: f64, t681: f64, t89: f64, t141997: f64, t142219: f64, t142224: f64, t142234: f64, t142240: f64, t1443: f64, t151347: f64, t1901: f64, t193: f64, t241: f64, t258: f64, t28141: f64, t28204: f64, t3281: f64, t3746: f64, t3898: f64, t724: f64, t7560: f64, t9707: f64, t97777: f64) -> (f64, f64, f64) {
    let t152164 = t2567 * t7440;
    let t152179 = t1882 * t35699;
    let t152191 = t1882 * t35734;
    let t152203 = t89 * t681 * t35684;
    let t152218 = -4.0_f64 / 9.0_f64 * t142219 + t152191 / 9.0_f64 + 2.0_f64 / 9.0_f64 * t142224 - 2.0_f64 / 9.0_f64 * t1901 * t97777 * t28204 - 2.0_f64 / 9.0_f64 * t142234 + 2.0_f64 / 9.0_f64 * t3281 * t724 * t7560 * t3746 - t152203 / 9.0_f64 + t142240 / 9.0_f64 + t1901 * t141997 * t3898 / 9.0_f64 - 4.0_f64 * t1901 * t9707 * t1443 * t28141 + t89 * t193 * t241 * t151347 * t258 / 3.0_f64;
    (t152164, t152179, t152218)
}
