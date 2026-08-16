//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2950/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2950(t1882: f64, t4056: f64, t2682: f64, t4000: f64, t5677: f64, t820: f64, t13985: f64, t46740: f64, t1872: f64, t3924: f64, t9816: f64, t9818: f64) -> (f64, f64, f64, f64) {
    let t48475 = t1882 * t4056;
    let t48486 = t820 * t4000 * t2682 * t5677;
    let t48488 = t46740 * t13985;
    let t48494 = t9816 * t9818 * t1872 * t3924;
    (t48475, t48486, t48488, t48494)
}
