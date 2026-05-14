//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 920/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk920<F: Float>(t114: F, t1509: F, t8315: F, t69: F, t8258: F, t8267: F, t8310: F, t8395: F, t8399: F) -> (F, F) {
    let t115 = 1.0 < t114;
    let t8402 = t8315 * t1509;
    let t8406 = piecewise3(t115, 0.0, t8310 + t8258 * t8395 / 4.0 + 5.0 / 24.0 * t69 * t8399 - 5.0 / 24.0 * t8267 * t8402);
    (t8402, t8406)
}
