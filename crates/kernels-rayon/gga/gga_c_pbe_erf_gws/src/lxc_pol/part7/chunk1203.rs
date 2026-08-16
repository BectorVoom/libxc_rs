//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1203/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1203(t2120: f64, t21385: f64, t2105: f64, t2112: f64, t2124: f64, t2387: f64, t6701: f64, t21011: f64, t21350: f64, t21355: f64, t21359: f64, t21361: f64, t21366: f64, t21378: f64, t21382: f64, t2277: f64, t2343: f64, t6195: f64, t6282: f64, t6366: f64, t6524: f64, t6609: f64, t904: f64, t929: f64, t9482: f64) -> (f64, f64, f64) {
    let t21387 = t2120 * t21385 / 96.0_f64;
    let t21388 = t2105 * t2112;
    let t21395 = t2387 * t6701 * t2124 / 16.0_f64;
    let t21396 = -7.0_f64 / 96.0_f64 * t21350 - t21355 + t21359 + 35.0_f64 / 128.0_f64 * t929 * t21361 * t904 * t21011 + t2277 * t9482 * t6609 * t21366 / 64.0_f64 - 5.0_f64 / 64.0_f64 * t2343 * t6366 * t6282 * t6524 - t21378 + t21382 - t21387 + t2277 * t9482 * t6195 * t21388 / 64.0_f64 - t21395;
    (t21387, t21395, t21396)
}
