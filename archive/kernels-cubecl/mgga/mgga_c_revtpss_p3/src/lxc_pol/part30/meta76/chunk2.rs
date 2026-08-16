//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 495/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk495<F: Float>(t1468: F, t100: F, t55: F, t108: F, t105: F, t109: F, t97: F, tau1: F) -> (F, F, F, F, F) {
    let t1504 = t1468 / F::cast_from(2.0_f64);
    let t1505 = t100 * t1504;
    let t1507 = tau1 * t55;
    let t1509 = -t1504;
    let t1510 = t108 * t1509;
    let t1513 = F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t105 * t1510 - F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t1507 * t109 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t97 * t1505;
    (t1504, t1505, t1507, t1509, t1513)
}
