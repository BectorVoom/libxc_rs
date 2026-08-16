//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1067/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1067(t2060: f64, t507: f64, t7811: f64, t31419: f64, t4810: f64, t721: f64, t30659: f64, t34610: f64, t34612: f64, t34614: f64, t34617: f64, t34618: f64, t34621: f64, t34623: f64, t34627: f64, t34630: f64, t34633: f64, t34636: f64, t34638: f64, t34640: f64, t34644: f64) -> f64 {
    let t34647 = t2060 * t507 * t7811;
    let t34650 = t31419 * t4810 * t721;
    let t34653 = -t34610 - t34612 + 0.21437009059034868486e-2_f64 * t34614 - t34617 - 0.11321313224257494744e-1_f64 * t34618 + t34621 - t34623 - t34627 + 0.64311027177104605458e-2_f64 * t34630 - t34633 - 0.47172138434406228102e-3_f64 * t34636 + 0.15724046144802076034e-3_f64 * t34638 + 0.28303283060643736862e-1_f64 * t34640 - 0.47172138434406228102e-2_f64 * t34644 + 0.7640625e-2_f64 * t34647 + 0.114609375e-1_f64 * t34650 + 0.25724410870841842183e-2_f64 * t30659;
    t34653
}
