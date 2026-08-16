//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 1080/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk1080(t1951: f64, t2039: f64, t270: f64, t638: f64, t1956: f64, t2046: f64, t2050: f64, t31: f64, t1954: f64, t2031: f64, t31227: f64, t36748: f64, t36753: f64, t36754: f64, t36756: f64, t41905: f64, t41922: f64, t41929: f64, t47676: f64, t47680: f64, t47690: f64, t47694: f64) -> f64 {
    let t47698 = t638 * t2039 * t1951 * t270;
    let t47702 = t638 * t2039 * t1956 * t270;
    let t47706 = t2046 * t2050 * t1956 * t31;
    let t47710 = t638 * t2039 * t1954 * t270;
    let t47714 = t2046 * t2050 * t1954 * t31;
    let t47716 = t41905 + 0.59590439850616975157e-4_f64 * t41922 + 0.15243824895787514157e-3_f64 * t47676 + 0.15243824895787514157e-3_f64 * t47680 - t41929 + 0.59871208509319042821e-1_f64 * t31227 * t2031 - 0.15243824895787514157e-3_f64 * t36748 - t36753 - 0.15243824895787514157e-3_f64 * t36754 + 0.30487649791575028314e-3_f64 * t36756 + 0.53205749866622299248e-5_f64 * t47690 + 0.21684485328539747656e-4_f64 * t47694 - 0.15243824895787514157e-3_f64 * t47698 - 0.30487649791575028314e-3_f64 * t47702 + 0.43368970657079495311e-4_f64 * t47706 - 0.15243824895787514157e-3_f64 * t47710 + 0.21684485328539747656e-4_f64 * t47714;
    t47716
}
