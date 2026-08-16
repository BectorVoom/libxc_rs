//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1409/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1409(t19680: f64, t70: f64, t18281: f64, t36: f64, t5826: f64, t627: f64, t1486: f64, t4181: f64, t4187: f64, t1470: f64, t4217: f64, t1494: f64, t21686: f64, t21687: f64, t21690: f64, t4182: f64, t5820: f64, t5827: f64, t5830: f64, t641: f64, t85: f64) -> f64 {
    let t21695 = t19680 * t70;
    let t21698 = t36 * t18281;
    let t21699 = t21698 * t70;
    let t21702 = t5826 * t627;
    let t21707 = t4181 * t1486;
    let t21710 = t4187 * t1486;
    let t21713 = t1470 * t4217;
    let t21720 = -t21686 * t21687 / 6.0_f64 - t21690 * t85 / 12.0_f64 - t5820 * t641 / 12.0_f64 - t21695 * t85 / 12.0_f64 - t21699 * t85 / 12.0_f64 - t21702 * t85 / 12.0_f64 - t5827 * t641 / 12.0_f64 - t21707 * t85 / 6.0_f64 - t21710 * t85 / 6.0_f64 - t21713 * t85 / 6.0_f64 - t5830 * t641 / 6.0_f64 - t4182 * t1494 / 6.0_f64;
    t21720
}
