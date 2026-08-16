//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2593/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2593(t1757: f64, t5180: f64, t1187: f64, t6538: f64, t6535: f64, t3523: f64, t6534: f64, t5184: f64, t12555: f64, t6518: f64, t12486: f64, t12553: f64, t17097: f64, t17154: f64, t20643: f64, t20647: f64, t20650: f64, t20654: f64, t20659: f64, t3496: f64, t3521: f64, t5163: f64, t5185: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t20662 = t1757 * t5180;
    let t20665 = t6538 * t1187;
    let t20668 = t6535 * t1187;
    let t20671 = t6534 * t3523;
    let t20672 = t20671 * t1187;
    let t20675 = t5184 * t5180;
    let t20678 = t6518 * t12555;
    let t20679 = t20678 * t1187;
    let t20682 = t20643 - t20647 - t20650 - t20654 - 0.23392894490538584828e1_f64 * t17154 * t5163 + 0.34631718211362927517e2_f64 * t17097 * t5185 + 0.35089341735807877242e1_f64 * t3521 * t20659 - 0.23392894490538584828e1_f64 * t3496 * t20662 - 0.10389515463408878255e3_f64 * t12486 * t20665 - 0.11696447245269292414e1_f64 * t3496 * t20668 + 0.17315859105681463759e2_f64 * t3521 * t20672 + 0.34631718211362927518e2_f64 * t3521 * t20675 + 0.10254018858216406658e4_f64 * t12553 * t20679;
    (t20662, t20665, t20668, t20671, t20672, t20675, t20678, t20679, t20682)
}
