//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 830/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk830(t6925: f64, t810: f64, t4545: f64, t2474: f64, t460: f64, t40: f64, t4757: f64, t950: f64, t1402: f64, t34: f64, t418: f64, t532: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6926 = t6925 * t810;
    let t6929 = 0.12654485932329694421e1_f64 * t4545;
    let t6930 = t2474 * t460;
    let t6931 = t40 * t6930;
    let t6932 = 2.0_f64 * t6931;
    let t6933 = t4757 * t950;
    let t6936 = t1402 * t34;
    let t6937 = t532 * t418;
    (t6926, t6929, t6932, t6933, t6936, t6937)
}
