//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 339/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk339<F: Float>(t1549: F, t1561: F, t1565: F, t797: F, t799: F, t812: F, t819: F, t825: F, t848: F, t851: F) -> F {
    let t1568 = -t797 - t799 * t1549 / F::new(48.0) - t812 + t819 - F::new(0.21437009059034868486e-3) * t825 * t1561 - t848 - F::new(0.85748036236139473944e-3) * t851 * t1565;
    t1568
}
