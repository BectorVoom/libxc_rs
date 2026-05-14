//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 207/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk207<F: Float>(t225: F, t860: F, t257: F, t213: F, t251: F, t256: F) -> (F, F, F, F, F) {
    let t861 = t860 * t225;
    let t862 = t861 * t257;
    let t865 = t213 * t251;
    let t866 = t256 * t256;
    let t867 = 1.0 / t866;
    (t861, t862, t865, t866, t867)
}
