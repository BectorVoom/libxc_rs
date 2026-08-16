//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta747 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2621;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2622;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta747(t47005: f64, t47007: f64, t13597: f64, t2626: f64, t5571: f64, t9387: f64, t47009: f64, t47011: f64, t47013: f64, t13613: f64, t2619: f64, t9323: f64, t47019: f64, t39773: f64, t39783: f64, t39786: f64, t39791: f64, t39795: f64, t39799: f64, t47003: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t48258, t48259, t48261, t48263, t48264, t48265, t48266, t48268, t48269) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2621(t47005, t47007, t13597, t2626, t5571, t9387, t47009, t47011, t47013, t13613, t2619, t9323);
        let (t48270, t48271, t48272) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2622(t48269, t47019, t39773, t39783, t39786, t39791, t39795, t39799, t47003, t48258, t48259, t48261, t48263, t48264, t48265, t48266, t48268);
    (t48258, t48259, t48261, t48263, t48264, t48265, t48266, t48268, t48270, t48271, t48272)
}
