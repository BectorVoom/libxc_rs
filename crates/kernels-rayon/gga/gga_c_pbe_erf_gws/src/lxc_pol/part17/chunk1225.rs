//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1225/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1225(t13796: f64, t3989: f64, t52921: f64, t875: f64, t1178: f64, t904: f64, t14637: f64, t9292: f64, t14688: f64, t2397: f64, t14802: f64, t2408: f64, t29751: f64, t50904: f64, t52889: f64, t52893: f64, t52897: f64, t52902: f64, t52904: f64, t52908: f64, t52910: f64, t52912: f64, t52917: f64, t52919: f64, t827: f64) -> (f64, f64) {
    let t52924 = t3989 * t13796 * t52921 * t875;
    let t52926 = t904 * t1178;
    let t52928 = t14637 * t52926 * t9292;
    let t52930 = t14688 * t2397;
    let t52931 = 7.0_f64 / 144.0_f64 * t52930;
    let t52935 = t52889 / 1536.0_f64 + t52893 / 32.0_f64 - t827 * t52897 / 48.0_f64 - 7.0_f64 / 144.0_f64 * t50904 - t52902 + t52904 / 768.0_f64 - t52908 / 192.0_f64 - t52910 / 48.0_f64 - t52912 / 48.0_f64 + t52917 / 192.0_f64 + t52919 / 48.0_f64 - t52924 / 1536.0_f64 - 5.0_f64 / 384.0_f64 * t52928 - t52931 - t2408 * t29751 * t14802 / 12.0_f64;
    (t52926, t52935)
}
