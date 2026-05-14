//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 841/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk841<F: Float>(t1988: F, t7763: F, t7767: F, t7770: F, t7799: F, t1980: F, t31032: F, t7476: F, t7693: F, t7658: F, t1017: F, t355: F, t3300: F, t7458: F, t1170: F, t31114: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t31164 = t1988 * t7763;
    let t31166 = t1988 * t7767;
    let t31168 = t7799 * t7770;
    let t31179 = t1980 * t7476 * t31032;
    let t31186 = t1988 * t7693;
    let t31188 = t1988 * t7658;
    let t31190 = t355 * t1017;
    let t31191 = t3300 * t31190;
    let t31193 = t1980 * t7458 * t31191;
    let t31195 = t1170 * t31114;
    (t31164, t31166, t31168, t31179, t31186, t31188, t31190, t31191, t31193, t31195)
}
