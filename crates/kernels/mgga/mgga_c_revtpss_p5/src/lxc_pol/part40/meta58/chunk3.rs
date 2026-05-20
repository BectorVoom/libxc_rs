//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 357/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk357<F: Float>(t1119: F, t1124: F, t422: F, t418: F) -> (F, F, F, F) {
    let t1126 = -t1119 + F::cast_from(0.17808333333333333333e-1_f64) * t1124;
    let t1128 = F::new(0.621814e-1) * t1126 * t422;
    let t1129 = t418 * t418;
    let t1130 = F::new(1.0) / t1129;
    (t1126, t1128, t1129, t1130)
}
