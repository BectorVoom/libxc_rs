//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1504/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1504(t14652: f64, t775: f64, t2430: f64, t4416: f64, t14468: f64, t832: f64, t14633: f64, t14643: f64, t14649: f64, t1553: f64, t1555: f64, t227: f64, t229: f64, t2634: f64, t2639: f64, t2642: f64, t4409: f64, t4415: f64, t4417: f64, t4420: f64, t830: f64, t833: f64) -> f64 {
    let t14653 = t14652 * t775;
    let t14656 = t4416 * t2430;
    let t14659 = t832 * t14468;
    let t14662 = -t14633 * t229 - 24.0_f64 * t14643 * t4417 + 60.0_f64 * t14649 * t4415 - 24.0_f64 * t14653 * t4415 - 12.0_f64 * t14656 * t4415 + 3.0_f64 * t14659 * t227 - 12.0_f64 * t1553 * t2639 + 3.0_f64 * t1553 * t2642 + 3.0_f64 * t1555 * t2634 + 6.0_f64 * t4409 * t833 + 6.0_f64 * t4420 * t830;
    t14662
}
