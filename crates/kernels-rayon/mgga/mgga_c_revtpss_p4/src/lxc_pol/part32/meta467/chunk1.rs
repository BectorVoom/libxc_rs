//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1694/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1694(t25944: f64, t26277: f64, t25950: f64, t7515: f64, t213: f64, t7506: f64, t2470: f64, t7514: f64) -> (f64, f64, f64, f64) {
    let t26279 = 0.17135234354032049604e-2_f64 * t25944 * t26277;
    let t26280 = t25950 * t7515;
    let t26282 = t213 * t7506;
    let t26292 = t7514 * t2470;
    (t26279, t26280, t26282, t26292)
}
