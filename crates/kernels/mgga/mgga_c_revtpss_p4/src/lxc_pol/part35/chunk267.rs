//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 267/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk267<F: Float>(t1179: F, t439: F, t1118: F, t1143: F, t447: F) -> (F, F, F, F) {
    let t1180 = t439 * t1179;
    let t1182 = F::cast_from(0.301925e0_f64) * t1118;
    let t1185 = F::cast_from(0.82785e-1_f64) * t1143;
    let t1188 = F::cast_from(1.0_f64) / t447;
    (t1180, t1182, t1185, t1188)
}
