//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1963/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1963(t2242: f64, t5826: f64, t19680: f64, t603: f64, t21663: f64, t607: f64, t5868: f64, t644: f64, t77: f64, t13269: f64, t1470: f64, t4173: f64, t4181: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t108762 = t2242 * t5826;
    let t108765 = t603 * t19680;
    let t108769 = t21663 * t607;
    let t108792 = t77 * t5868 * t644;
    let t108807 = t13269 * t1470;
    let t108810 = t4173 * t4181;
    (t108762, t108765, t108769, t108792, t108807, t108810)
}
