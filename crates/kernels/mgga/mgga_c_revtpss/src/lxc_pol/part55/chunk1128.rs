//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1128/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1128<F: Float>(t119912: F, t31834: F, t846: F, t240: F, t822: F, t843: F, t31752: F, t31758: F, t1032: F, t786: F, t119835: F, t119893: F) -> (F, F, F, F, F, F, F) {
    let t119913 = F::cast_from(0.34708173928447610098e-2_f64) * t119912;
    let t119914 = t31834 * t846;
    let t119915 = F::cast_from(0.17354086964223805049e-2_f64) * t119914;
    let t119934 = t822 * t843 * t240;
    let t119935 = t31752 * t119934;
    let t119936 = t119935 * t31758;
    let t119937 = F::cast_from(0.263521689745817692e-2_f64) * t119936;
    let t119967 = t786 * t1032;
    let t119968 = t119967 * t119835;
    let t119969 = t119968 * t119893;
    (t119913, t119915, t119935, t119937, t119967, t119968, t119969)
}
