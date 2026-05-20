//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 461/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk461<F: Float>(t108: F, t1509: F, t105: F, t109: F, t1505: F, t1507: F, t97: F) -> F {
    let t1510 = t108 * t1509;
    let t1513 = F::new(5.0) / F::new(3.0) * t105 * t1510 - F::new(5.0) / F::new(3.0) * t1507 * t109 + F::new(5.0) / F::new(3.0) * t97 * t1505;
    t1513
}
