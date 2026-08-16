//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1202/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1202(t13610: f64, t13638: f64, t13663: f64, t14308: f64, t1532: f64, t2609: f64, t10437: f64, t2398: f64, t4308: f64, t4305: f64, t262: f64, t4343: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14310 = t13610 + t13638 + t13663 + t14308;
    let t14312 = t1532 * t2609;
    let t14313 = 4.0_f64 * t10437;
    let t14315 = 8.0_f64 * t2398 * t4308;
    let t14317 = 8.0_f64 * t2398 * t4305;
    let t14318 = t262 * t4343;
    (t14310, t14312, t14313, t14315, t14317, t14318)
}
