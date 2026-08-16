//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 760/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk760(t343: f64, t6231: f64, t904: f64, t916: f64, t2277: f64, t2312: f64, t6182: f64, t6186: f64, t6190: f64, t6192: f64, t6198: f64, t6204: f64, t6208: f64, t6213: f64, t6219: f64, t6224: f64, t6225: f64, t6230: f64, t914: f64) -> (f64, f64, f64) {
    let t6232 = t6231 * t343;
    let t6234 = t916 * t904 * t6232;
    let t6237 = -t6182 + t6186 - t6190 - t2312 * t6192 / 128.0_f64 - t2277 * t6198 / 256.0_f64 + 7.0_f64 / 96.0_f64 * t6204 - t2312 * t6208 / 128.0_f64 - t2312 * t6213 / 128.0_f64 - t6219 + t6224 - 7.0_f64 / 768.0_f64 * t6225 - t6230 - t914 * t6234 / 1536.0_f64;
    (t6232, t6234, t6237)
}
