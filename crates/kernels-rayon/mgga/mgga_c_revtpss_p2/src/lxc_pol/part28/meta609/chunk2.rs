//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2121/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2121(t98134: f64, t98158: f64, t98184: f64, t98208: f64, t98233: f64, t98255: f64, t98271: f64, t98287: f64, t543: f64, t97870: f64, t27857: f64, t689: f64) -> (f64, f64, f64) {
    let t98290 = t98134 + t98158 + t98184 + t98208 + t98233 + t98255 + t98271 + t98287;
    let t98299 = t97870 * t543;
    let t98303 = t27857 * t689;
    (t98290, t98299, t98303)
}
