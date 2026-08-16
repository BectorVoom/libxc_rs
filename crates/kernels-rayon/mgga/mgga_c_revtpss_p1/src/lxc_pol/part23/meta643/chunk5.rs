//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2367/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2367(t39913: f64, t39957: f64, t40007: f64, t40080: f64, t158: f64, t162: f64, t2492: f64, t9417: f64, t9507: f64, t760: f64, t2523: f64, t9323: f64) -> (f64, f64, f64, f64, f64) {
    let t40082 = t39913 + t39957 + t40007 + t40080;
    let t40084 = t158 * t162 * t40082;
    let t40086 = t9417 * t2492 * t9507;
    let t40088 = 0.62337092780453269531e3_f64 * t760 * t40086;
    let t40092 = t2523 * t9323;
    (t40082, t40084, t40086, t40088, t40092)
}
