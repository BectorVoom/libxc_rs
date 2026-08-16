//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1062/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1062(t1158: f64, t6505: f64, t8574: f64, t904: f64, t933: f64, t2312: f64, t2343: f64, t9175: f64, t9177: f64, t929: f64, t9626: f64, t9632: f64, t9634: f64, t9637: f64, t9641: f64, t9645: f64, t9647: f64, t9651: f64, t9655: f64) -> (f64, f64) {
    let t9658 = t6505 * t1158;
    let t9661 = t933 * t904 * t8574;
    let t9664 = t2343 * t9626 / 384.0_f64 - t9632 - t2343 * t9634 / 1536.0_f64 + t9637 * t9641 / 128.0_f64 + t9175 - t9645 - 5.0_f64 / 128.0_f64 * t929 * t9647 - t9177 + t2312 * t9651 / 192.0_f64 - t2312 * t9655 / 192.0_f64 - 119.0_f64 / 3456.0_f64 * t9658 - t929 * t9661 / 768.0_f64;
    (t9661, t9664)
}
