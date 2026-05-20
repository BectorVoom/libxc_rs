//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 527/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk527<F: Float>(t1312: F, t2199: F, t2201: F, t651: F, t3: F, param_d: F) -> (F, F, F) {
    let t2204 = F::new(2.0) * t1312 * t2201 - F::new(2.0) * t2199 * t651;
    let t2205 = t3 * t2204;
    let t2207 = param_d * t2204;
    (t2204, t2205, t2207)
}
