//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 588/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk588<F: Float>(t118: F, t2014: F, t2052: F, t2056: F, t2089: F, t2093: F, t2108: F, t508: F, t569: F, t651: F, t3: F, param_d: F) -> (F, F, F) {
    let t2110 = -t118 * t2089 + t2014 * t2108 - t2052 * t508 - F::new(2.0) * t2056 * t651 + t2093 * t569;
    let t2111 = t3 * t2110;
    let t2113 = param_d * t2110;
    (t2110, t2111, t2113)
}
