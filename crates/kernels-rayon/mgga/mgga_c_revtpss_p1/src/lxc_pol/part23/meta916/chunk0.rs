//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2955/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2955(t15547: f64, t6223: f64, t1642: f64, t64510: f64, t23453: f64, t3022: f64, t1100: f64, t23571: f64, t41937: f64, t5023: f64, t77634: f64, t77636: f64, t77639: f64, t77641: f64, t77643: f64, t77645: f64, t77647: f64) -> (f64, f64, f64, f64) {
    let t78405 = 0.17544670867903938621e1_f64 * t15547 * t6223;
    let t78411 = 0.17544670867903938621e1_f64 * t64510 * t1642;
    let t78413 = 0.10389515463408878255e3_f64 * t3022 * t23453;
    let t78414 = -6.0_f64 * t1100 * t23571 * t41937 * t5023 + t77634 - t77636 + t77639 + t77641 + t77643 - t77645 + t77647 - t78405 - t78411 + t78413;
    (t78405, t78411, t78413, t78414)
}
