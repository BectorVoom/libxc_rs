//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2886/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2886(t2: f64, t580: f64, t6084: f64, t19049: f64, t4729: f64, t23649: f64, t3022: f64, t19023: f64, t4719: f64, t23457: f64, t23478: f64, t689: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t77481 = 3.0_f64 * t6084 * t2 * t580;
    let t77492 = 0.17544670867903938621e1_f64 * t19049 * t4729;
    let t77494 = 0.10254018858216406658e4_f64 * t3022 * t23649;
    let t77496 = 0.17544670867903938621e1_f64 * t4719 * t19023;
    let t77498 = 0.35089341735807877242e1_f64 * t3022 * t23457;
    let t77499 = t689 * t23478;
    (t77481, t77492, t77494, t77496, t77498, t77499)
}
