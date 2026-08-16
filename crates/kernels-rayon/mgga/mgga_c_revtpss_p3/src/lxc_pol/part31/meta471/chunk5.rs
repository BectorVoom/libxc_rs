//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1725/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1725(t545: f64, t6888: f64, t869: f64, t689: f64, t22005: f64, t4003: f64, t5744: f64, t2782: f64, t21981: f64, t4086: f64, t543: f64, t22009: f64) -> (f64, f64, f64, f64) {
    let t22351 = t545 * t6888;
    let t22352 = t869 * t22351;
    let t22353 = t689 * t22352;
    let t22361 = t5744 * t22005 * t4003;
    let t22362 = t2782 * t22361;
    let t22365 = t4086 * t21981 * t543;
    let t22366 = t2782 * t22365;
    let t22369 = t4086 * t22009 * t543;
    (t22353, t22362, t22366, t22369)
}
