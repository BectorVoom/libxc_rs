//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2644/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2644(t1872: f64, t4057: f64, t9816: f64, t9818: f64, t13824: f64, t221: f64, t3978: f64, t46716: f64, t13923: f64, t3930: f64, t14036: f64, t9976: f64) -> (f64, f64, f64, f64) {
    let t48655 = t9816 * t9818 * t1872 * t4057;
    let t48662 = t221 * t13824;
    let t48664 = t3978 * t46716 * t48662;
    let t48666 = t3930 * t13923;
    let t48668 = t9976 * t14036;
    (t48655, t48664, t48666, t48668)
}
