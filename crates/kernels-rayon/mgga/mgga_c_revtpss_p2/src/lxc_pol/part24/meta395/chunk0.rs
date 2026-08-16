//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1311/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1311(t2783: f64, t9646: f64, t10111: f64, t588: f64, t870: f64, t2434: f64, t2626: f64, t2629: f64, t676: f64, t9425: f64, t2567: f64, t2576: f64, t2582: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t39698 = t9646 * t2783;
    let t39723 = 0.15709759505761725819e-2_f64 * t10111 * t870 * t588;
    let t39739 = t2434 * t2626;
    let t39741 = 0.86748650402413918736e-1_f64 * t2629 * t39739;
    let t39742 = t676 * t9425;
    let t39744 = 0.1301229756036208781e0_f64 * t2629 * t39742;
    let t39747 = 36.0_f64 * t2582 * t2567 * t2576;
    (t39698, t39723, t39739, t39741, t39742, t39744, t39747)
}
