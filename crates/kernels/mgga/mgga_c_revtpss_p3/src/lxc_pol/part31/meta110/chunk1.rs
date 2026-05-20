//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 653/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk653<F: Float>(t2619: F, t755: F, t72: F, t752: F, t757: F, t2492: F, t2596: F, t745: F) -> (F, F, F, F) {
    let t2621 = F::cast_from(0.24415263074675393405e-3_f64) * t755 * t2619;
    let t2622 = t752 * t72;
    let t2623 = t2622 * t757;
    let t2626 = t2596 * t2492 * t745;
    (t2621, t2622, t2623, t2626)
}
