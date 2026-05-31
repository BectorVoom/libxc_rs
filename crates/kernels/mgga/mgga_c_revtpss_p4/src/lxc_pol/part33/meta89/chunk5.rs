//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 577/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk577<F: Float>(t1945: F, t213: F, t248: F, t1943: F) -> (F, F) {
    let t1946 = t213 * t1945;
    let t1947 = t1946 * t248;
    let t1949 = t1943 / F::cast_from(96.0_f64) + F::cast_from(0.42874018118069736972e-3_f64) * t1947;
    (t1946, t1949)
}
