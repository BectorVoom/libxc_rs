//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 1016/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk1016(t118573: f64, t112784: f64, t114714: f64, t114720: f64, t118533: f64, t118535: f64, t118539: f64, t118546: f64, t118549: f64, t118552: f64, t118556: f64, t118559: f64, t118562: f64, t118566: f64, t118569: f64, t118576: f64, t118578: f64, t118580: f64) -> f64 {
    let t123566 = 0.32298204875312312682e-2_f64 * t118573;
    let t123570 = -t118533 / 384.0_f64 - t118535 / 384.0_f64 - t118539 / 384.0_f64 + 5.0_f64 / 96.0_f64 * t118546 - 0.32298204875312312682e-2_f64 * t118549 + 0.13565246047631171326e0_f64 * t118552 + t114714 + 0.64596409750624625364e-2_f64 * t118556 + 0.19378922925187387609e-1_f64 * t118559 + 0.13565246047631171326e0_f64 * t112784 + t118562 / 192.0_f64 + t114720 + 0.19378922925187387609e-1_f64 * t118566 - 0.32298204875312312682e-2_f64 * t118569 + t123566 + t118576 / 384.0_f64 + 0.22608743412718618877e-1_f64 * t118578 + 0.13565246047631171326e0_f64 * t118580;
    t123570
}
