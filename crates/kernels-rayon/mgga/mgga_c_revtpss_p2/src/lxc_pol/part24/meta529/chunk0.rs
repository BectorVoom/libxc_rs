//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1564/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1564(t17605: f64, t21090: f64, t127: f64, t12988: f64, t24617: f64, t371: f64, t20842: f64, t5323: f64, t1010: f64, t22700: f64, t21169: f64, t5373: f64) -> (f64, f64, f64, f64, f64) {
    let t83916 = t17605 * t21090;
    let t83920 = t12988 * t371 * t127 * t24617;
    let t83922 = t5323 * t20842;
    let t83962 = t22700 * t1010;
    let t83992 = t5373 * t21169;
    (t83916, t83920, t83922, t83962, t83992)
}
