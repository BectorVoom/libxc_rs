//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 279/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk279<F: Float>(t233: F, t251: F, t869: F, t689: F, t234: F, t786: F) -> (F, F, F, F) {
    let t870 = t233 * t251;
    let t871 = t869 * t870;
    let t873 = F::cast_from(0.54878743191129263322e-2_f64) * t689 * t871;
    let t874 = t786 * t234;
    (t870, t871, t873, t874)
}
