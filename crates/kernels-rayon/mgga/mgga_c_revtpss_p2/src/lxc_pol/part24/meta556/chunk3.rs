//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1664/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1664(t41307: f64, t51978: f64, t77736: f64, t88118: f64, t88126: f64, t88134: f64, t88168: f64, t88171: f64, t88203: f64, t88206: f64, t88209: f64, t88211: f64, t88214: f64, t88216: f64) -> f64 {
    let t88218 = 0.198684e1_f64 * t88168 + 0.49671e0_f64 * t88171 + t41307 + 0.132456e1_f64 * t77736 + 0.12524296296296296297e1_f64 * t51978 - 0.20128333333333333334e1_f64 * t88118 + 0.72462e1_f64 * t88126 - 0.80513333333333333332e0_f64 * t88134 + 0.258925e1_f64 * t88203 - 0.485484375e1_f64 * t88206 - 0.11038e0_f64 * t88209 + 0.6189328125e-1_f64 * t88211 + 0.247573125e0_f64 * t88214 - 0.3883875e1_f64 * t88216;
    t88218
}
