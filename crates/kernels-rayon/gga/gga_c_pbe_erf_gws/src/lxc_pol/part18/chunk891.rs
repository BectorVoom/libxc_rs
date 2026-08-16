//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 891/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk891(t829: f64, t830: f64, t9883: f64, t3717: f64, t831: f64, t2370: f64, t1115: f64, t2397: f64, t2408: f64, t3207: f64, t335: f64, t3917: f64, t4425: f64, t4430: f64, t4443: f64, t827: f64, t8622: f64, t8641: f64, t8643: f64, t8646: f64, t8664: f64, t8666: f64, t8710: f64, t9865: f64, t9869: f64, t9873: f64, t9879: f64) -> (f64, f64) {
    let t9885 = t829 * t830 * t9883;
    let t9888 = t831 * t3717;
    let t9890 = t2370 * t830 * t9888;
    let t9893 = t3917 * t2397 / 96.0_f64 + t335 * t9865 / 48.0_f64 + t8622 + t2408 * t9869 / 24.0_f64 - t3207 * t9873 / 8.0_f64 + 35.0_f64 / 432.0_f64 * t4425 - 35.0_f64 / 432.0_f64 * t4430 - 35.0_f64 / 216.0_f64 * t4443 - 7.0_f64 / 144.0_f64 * t9879 + t8641 + t8643 + t8646 + t8664 - t1115 * t8710 / 24.0_f64 - t827 * t9885 / 48.0_f64 - t827 * t9890 / 48.0_f64 - t8666;
    (t9888, t9893)
}
