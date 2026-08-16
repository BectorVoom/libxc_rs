//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 870/1429 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk870(t1: f64, t277: f64, t9060: f64, t2546: f64, t3328: f64, t2210: f64, t2767: f64, t3045: f64, t7294: f64, t3120: f64, t3363: f64, t1089: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9894 = t277 * t1;
    let t9895 = t9894 * t9060;
    let t9896 = t2546 * t3328;
    let t9897 = t2210 * t9896;
    let t9898 = t9895 * t9897;
    let t9901 = t7294 * t3045 * t2767;
    let t9903 = t3363 * t3120;
    let t9904 = t9903 * t1089;
    (t9894, t9895, t9896, t9897, t9898, t9901, t9903, t9904)
}
