//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1179/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1179<F: Float>(t119859: F, t27279: F, t126138: F, t2747: F, t31767: F, t31772: F, t119928: F, t126221: F, t119894: F, t119904: F, t119912: F, t119914: F, t119920: F, t119931: F, t119936: F, t119957: F, t119960: F, t119966: F, t120058: F, t126208: F, t126210: F, t126214: F, t126222: F, t126226: F, t14495: F, t31812: F, t32463: F, t4533: F, t8471: F, t8649: F) -> F {
    let t126228 = t119859 * t27279;
    let t126232 = t31767 * t2747 * t31772 * t126138;
    let t126239 = t119928 * t126221;
    let t126241 = -F::new(0.33467254597718846885e-4) * t119894 + t119904 - F::new(0.17354086964223805049e-2) * t119912 - F::new(0.76169170176413987216e-1) * t126208 + F::new(0.131760844872908846e-2) * t126210 + F::new(0.86770434821119025247e-3) * t119914 + F::new(0.3718732920905101082e-3) * t126214 - F::new(0.17135921299530705785e1) * t8649 * t31812 * t8471 * t4533 + F::new(0.51405703062096148813e-1) * t126222 - F::new(0.14279934416275588154e-1) * t119920 - F::new(0.17354086964223805049e-2) * t126226 + F::new(0.28912093960683998207e-1) * t126228 + F::new(0.112937867033921868e-2) * t126232 - F::new(0.28912093960683998208e-1) * t119931 - F::new(0.11423947533020470523e1) * t32463 * t120058 * t14495 + F::new(0.131760844872908846e-2) * t119936 - F::new(0.28912093960683998207e-1) * t126239 + t119957 - t119960 + t119966;
    t126241
}
