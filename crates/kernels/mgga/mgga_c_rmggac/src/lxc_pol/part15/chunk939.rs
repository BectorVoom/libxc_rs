//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 939/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk939<F: Float>(t1956: F, t2039: F, t270: F, t638: F, t2046: F, t2050: F, t31: F, t1954: F, t2031: F, t31227: F, t36748: F, t36753: F, t36754: F, t36756: F, t41905: F, t41922: F, t41929: F, t47676: F, t47680: F, t47690: F, t47694: F, t47698: F) -> (F,) {
    let t47702 = t638 * t2039 * t1956 * t270;
    let t47706 = t2046 * t2050 * t1956 * t31;
    let t47710 = t638 * t2039 * t1954 * t270;
    let t47714 = t2046 * t2050 * t1954 * t31;
    let t47716 = t41905 + 0.59590439850616975157e-4 * t41922 + 0.15243824895787514157e-3 * t47676 + 0.15243824895787514157e-3 * t47680 - t41929 + 0.59871208509319042821e-1 * t31227 * t2031 - 0.15243824895787514157e-3 * t36748 - t36753 - 0.15243824895787514157e-3 * t36754 + 0.30487649791575028314e-3 * t36756 + 0.53205749866622299248e-5 * t47690 + 0.21684485328539747656e-4 * t47694 - 0.15243824895787514157e-3 * t47698 - 0.30487649791575028314e-3 * t47702 + 0.43368970657079495311e-4 * t47706 - 0.15243824895787514157e-3 * t47710 + 0.21684485328539747656e-4 * t47714;
    (t47716,)
}
