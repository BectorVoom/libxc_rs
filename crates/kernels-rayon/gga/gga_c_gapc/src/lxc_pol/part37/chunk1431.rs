//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1431/1445 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1431(t33507: f64, t36609: f64, t36610: f64, t36611: f64, t36612: f64, t36613: f64, t36615: f64, t36616: f64, t36617: f64, t36618: f64, t36619: f64, t33541: f64, t33552: f64, t33565: f64, t36621: f64, t36623: f64, t36625: f64, t36626: f64, t36627: f64, t36628: f64, t36630: f64, t36631: f64) -> (f64, f64) {
    let t38743 = -t36609 - t36610 + t36611 - t36612 + t36613 + 0.67632724766374884054e-5_f64 * t33507 - t36615 - t36616 + t36617 + t36618 - t36619;
    let t38747 = t36621 - 0.53808777420609085649e-7_f64 * t33541 + t36623 - 0.89048050908546122982e-5_f64 * t33552 - t36625 + t36626 - t36627 - t36628 + 0.12650553385416666667e-5_f64 * t33565 + t36630 + t36631;
    (t38743, t38747)
}
