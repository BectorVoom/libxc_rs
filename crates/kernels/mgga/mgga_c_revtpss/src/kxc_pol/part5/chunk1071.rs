//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1071/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1071<F: Float>(t18281: F, t190: F, t706: F, t14441: F, t10593: F, t10597: F, t189: F, t5819: F, t606: F, t14330: F, t10608: F, t4308: F, t4311: F, t10613: F, t10592: F, t10596: F, t10604: F, t10611: F, t14433: F, t14618: F, t9524: F, t9542: F) -> (F, F, F, F, F, F, F, F, F) {
    let t18569 = t190 * t18281;
    let t18571 = 4.0 * t706 * t18569;
    let t18572 = 8.0 * t14441;
    let t18573 = 0.5848223622634646207e0 * t10593;
    let t18574 = 0.17315859105681463759e2 * t10597;
    let t18575 = t189 * t5819;
    let t18576 = t18575 * t606;
    let t18578 = 24.0 * t14330 * t18576;
    let t18579 = 0.11696447245269292414e1 * t10608;
    let t18581 = 8.0 * t4311 * t4308;
    let t18582 = 4.0 * t10613;
    let t18583 = t14433 + t18571 - t9524 + t10592 + t18572 - t18573 - t10596 - t18574 + t18578 - t10604 + t9542 - t14618 + t18579 + t18581 - t10611 + t18582;
    (t18571, t18572, t18573, t18574, t18578, t18579, t18581, t18582, t18583)
}
