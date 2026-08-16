//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1033/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1033<F: Float>(t251: F, t4503: F, t786: F, t2453: F, t2797: F, t231: F, t281: F, t68: F, t836: F, t2783: F, t860: F, t760: F, t9323: F) -> (F, F, F, F, F, F) {
    let t10529 = t4503 * t251;
    let t10530 = t786 * t10529;
    let t10535 = t2453 * t2797;
    let t10538 = t281 * t68 * t836 * t231;
    let t10539 = t10535 * t10538;
    let t10541 = t2783 * t860;
    let t10542 = t786 * t10541;
    let t10552 = F::cast_from(0.51947577317044391277e2_f64) * t760 * t9323;
    (t10529, t10530, t10535, t10539, t10542, t10552)
}
