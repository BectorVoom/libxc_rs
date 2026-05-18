//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 1084/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk1084<F: Float>(t9088: F, t9093: F, t9102: F, t9112: F, t9114: F, t9119: F, t37039: F, t7913: F, t7916: F, t7918: F, t8222: F, t9107: F, t9636: F) -> (F, F, F) {
    let t42316 = F::new(0.85129199786595678796e-5) * t9088;
    let t42317 = F::new(0.39914139006212695214e-1) * t9093;
    let t42320 = F::new(0.11974241701863808564e0) * t9102;
    let t42322 = F::new(0.85129199786595678796e-5) * t9112;
    let t42323 = F::new(0.85129199786595678796e-5) * t9114;
    let t42324 = F::new(0.31923449919973379548e-4) * t9119;
    let t42325 = -t8222 - t7913 - t42320 + t7916 + t7918 + F::new(0.25538759935978703638e-4) * t9107 + t42322 - t42323 - t42324 + t9636 + t37039;
    (t42316, t42317, t42325)
}
