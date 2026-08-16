//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1154/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1154<F: Float>(t7038: F, t839: F, t1946: F, t846: F, t233: F, t64: F) -> (F, F, F) {
    let t7039 = t7038 * t839;
    let t7041 = t1946 * t846;
    let t7042 = F::cast_from(0.20007875121765877254e-2_f64) * t7041;
    let t7043 = t233 * t64;
    (t7039, t7042, t7043)
}
