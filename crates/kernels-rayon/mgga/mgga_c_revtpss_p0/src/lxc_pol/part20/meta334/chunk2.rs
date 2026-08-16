//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1255/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1255(t12663: f64, t13189: f64, t12413: f64, t12417: f64, t12566: f64, t12573: f64, t12575: f64, t12577: f64, t12579: f64, t12583: f64, t12584: f64, t12587: f64, t12594: f64, t12598: f64, t1298: f64, t1300: f64, t198: f64, t336: f64, t3794: f64, t3801: f64, t5023: f64) -> (f64, f64) {
    let t13190 = t12663 + t13189;
    let t13194 = 2.0_f64 * t12584 * t12587 * t198 * t336 - 3.0_f64 * t1298 * t3794 * t3801 * t5023 + t1300 * t13190 * t198 * t336 - t12413 + t12417 - t12566 - t12573 - t12575 - t12577 + t12579 + t12583 - t12594 - t12598;
    (t13190, t13194)
}
