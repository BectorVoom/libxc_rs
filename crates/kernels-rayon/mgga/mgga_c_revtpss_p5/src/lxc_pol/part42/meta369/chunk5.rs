//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1201/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1201(t18426: f64, t4364: f64, t4366: f64, t2741: f64, t5980: f64, t4365: f64, t4424: f64, t837: f64, t125: f64, t5966: f64, t10770: f64, t2652: f64, t5993: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18456 = t4364 * t18426 * t4366;
    let t18459 = t2741 * t5980;
    let t18462 = t4364 * t4365 * t4424;
    let t18466 = t4364 * t18426 * t837;
    let t18469 = t125 * t5966;
    let t18471 = t10770 * t18469 * t837;
    let t18475 = t2652 * t5993;
    (t18456, t18459, t18462, t18466, t18471, t18475)
}
