//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2881/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2881(t5966: f64, t890: f64, t18435: f64, t18498: f64, t39989: f64, t40150: f64, t4541: f64, t4546: f64, t4556: f64, t50098: f64, t77007: f64, t77008: f64, t77009: f64, t77010: f64, t77011: f64) -> f64 {
    let t77408 = t5966 * t890;
    let t77412 = 18.0_f64 * t18435 * t4541 * t4546 + 36.0_f64 * t18498 * t4541 * t4546 - 18.0_f64 * t4541 * t4556 * t77408 - t39989 + t40150 + t50098 + t77007 + t77008 + t77009 + t77010 - t77011;
    t77412
}
