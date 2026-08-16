//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 826/1327 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk826(t2660: f64, t2767: f64, t8639: f64, t1081: f64, t2807: f64, t2752: f64, t2685: f64, t3357: f64, t3360: f64, t1: f64, t277: f64, t9060: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9881 = t2660 * t8639 * t2767;
    let t9883 = t1081 * t2807;
    let t9885 = t1081 * t2752;
    let t9887 = t3357 * t2685;
    let t9889 = t3360 * t2685;
    let t9894 = t277 * t1;
    let t9895 = t9894 * t9060;
    (t9881, t9883, t9885, t9887, t9889, t9894, t9895)
}
