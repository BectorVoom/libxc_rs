//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1189/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1189<F: Float>(t1187: F, t6535: F, t3523: F, t6534: F, t5180: F, t5184: F, t12555: F, t6518: F, t12486: F, t12553: F, t17097: F, t17154: F, t20643: F, t20647: F, t20650: F, t20654: F, t20659: F, t20662: F, t20665: F, t3496: F, t3521: F, t5163: F, t5185: F) -> (F,) {
    let t20668 = t6535 * t1187;
    let t20671 = t6534 * t3523;
    let t20672 = t20671 * t1187;
    let t20675 = t5184 * t5180;
    let t20678 = t6518 * t12555;
    let t20679 = t20678 * t1187;
    let t20682 = t20643 - t20647 - t20650 - t20654 - 0.23392894490538584828e1 * t17154 * t5163 + 0.34631718211362927517e2 * t17097 * t5185 + 0.35089341735807877242e1 * t3521 * t20659 - 0.23392894490538584828e1 * t3496 * t20662 - 0.10389515463408878255e3 * t12486 * t20665 - 0.11696447245269292414e1 * t3496 * t20668 + 0.17315859105681463759e2 * t3521 * t20672 + 0.34631718211362927518e2 * t3521 * t20675 + 0.10254018858216406658e4 * t12553 * t20679;
    (t20682,)
}
