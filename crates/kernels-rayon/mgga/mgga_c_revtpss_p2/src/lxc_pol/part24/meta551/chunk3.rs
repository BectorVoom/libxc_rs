//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1641/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1641(t41235: f64, t41238: f64, t88008: f64, t981: f64, t11509: f64, t41224: f64, t6141: f64, t2874: f64, t935: f64, t2924: f64, t2926: f64, t6110: f64, t63677: f64) -> (f64, f64, f64, f64, f64) {
    let t88012 = 0.91082604192152556044e5_f64 * t981 * t41235 * t88008 * t41238;
    let t88016 = 0.12304822629859687989e5_f64 * t981 * t41224 * t88008 * t11509;
    let t88020 = t6141 * t6141;
    let t88023 = 6.0_f64 * t2874 * t88020 * t935;
    let t88026 = 0.48245938496077605201e2_f64 * t2924 * t88020 * t2926;
    let t88028 = 12.0_f64 * t63677 * t6110;
    (t88012, t88016, t88023, t88026, t88028)
}
