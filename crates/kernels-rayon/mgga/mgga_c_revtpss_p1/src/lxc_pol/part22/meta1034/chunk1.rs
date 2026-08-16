//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3619/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3619(t2439: f64, t6464: f64, t1145: f64, t141: f64, t68251: f64, t6461: f64, t3417: f64, t68395: f64, t58209: f64, t58211: f64, t58225: f64, t68456: f64, t68459: f64, t68567: f64, t68570: f64, t68573: f64, t68576: f64, t68578: f64, t68583: f64) -> (f64, f64, f64, f64, f64) {
    let t68585 = t2439 * t6464;
    let t68588 = t141 * t1145 * t68251;
    let t68590 = t2439 * t6461;
    let t68593 = t141 * t3417 * t68395;
    let t68595 = -0.12077e1_f64 * t68456 + 0.181155e1_f64 * t68459 - 0.11038e0_f64 * t68567 + 0.82785e-1_f64 * t68570 - 0.5519e-1_f64 * t68573 - 0.27595e-1_f64 * t68576 + 0.16504875e0_f64 * t68578 - 0.22076e0_f64 * t58209 - 0.66228e0_f64 * t58211 + 0.73586666666666666667e0_f64 * t58225 + 0.91983333333333333334e-1_f64 * t68583 + 0.18396666666666666667e0_f64 * t68585 + 0.33114e0_f64 * t68588 - 0.30661111111111111112e-1_f64 * t68590 - 0.5519e-1_f64 * t68593;
    (t68585, t68588, t68590, t68593, t68595)
}
