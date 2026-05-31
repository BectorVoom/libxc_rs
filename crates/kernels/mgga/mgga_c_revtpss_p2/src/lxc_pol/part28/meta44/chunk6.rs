//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 297/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk297<F: Float>(t797: F, t799: F, t802: F, t812: F, t819: F, t825: F, t839: F, t848: F, t851: F, t857: F) -> F {
    let t860 = -t797 - t799 * t802 / F::cast_from(48.0_f64) - t812 + t819 - F::cast_from(0.21437009059034868486e-3_f64) * t825 * t839 - t848 - F::cast_from(0.85748036236139473944e-3_f64) * t851 * t857;
    t860
}
