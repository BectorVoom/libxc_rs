//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2640/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2640(t13910: f64, t1399: f64, t2661: f64, t3992: f64, t4057: f64, t5651: f64, t1389: f64, t1882: f64, t46856: f64, t543: f64, t685: f64, t72: f64) -> (f64, f64, f64) {
    let t48553 = t2661 * t3992 * t13910 * t1399;
    let t48557 = t2661 * t3992 * t5651 * t4057;
    let t48563 = t46856 * t1389 * t1882 * t543 * t72 * t685;
    (t48553, t48557, t48563)
}
