//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1222/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1222(t14673: f64, t2397: f64, t3165: f64, t376: f64, t13796: f64, t3989: f64, t875: f64, t1178: f64, t904: f64, t14637: f64, t9292: f64, t14688: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t52919 = t14673 * t2397;
    let t52921 = t376 * t3165;
    let t52924 = t3989 * t13796 * t52921 * t875;
    let t52926 = t904 * t1178;
    let t52928 = t14637 * t52926 * t9292;
    let t52930 = t14688 * t2397;
    (t52919, t52921, t52924, t52926, t52928, t52930)
}
