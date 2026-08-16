//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1964/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1964(t21906: f64, t3403: f64, t11369: f64, t11372: f64, t14702: f64, t14766: f64, t18203: f64, t18219: f64, t18229: f64, t18494: f64, t18505: f64, t18512: f64, t21739: f64, t21741: f64, t21747: f64, t21751: f64) -> (f64, f64) {
    let t21907 = t21906 * t3403;
    let t21922 = -t11369 - 0.16557e0_f64 * t18512 + 0.20128333333333333333e0_f64 * t18203 - 0.60385000000000000001e0_f64 * t18219 - 0.30192500000000000001e0_f64 * t18229 + 0.5519e-1_f64 * t18494 - 0.33114e0_f64 * t18505 - 0.3883875e1_f64 * t21739 + 0.247573125e0_f64 * t21741 - t11372 + 0.40256666666666666668e0_f64 * t14702 + 0.27595e0_f64 * t14766 - 0.82785e-1_f64 * t21747 + 0.49671e0_f64 * t21751;
    (t21907, t21922)
}
