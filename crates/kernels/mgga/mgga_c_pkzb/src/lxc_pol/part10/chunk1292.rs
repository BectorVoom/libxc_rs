//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1292/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1292<F: Float>(t20659: F, t20918: F, t683: F, t3618: F, t5754: F, t1972: F, t3625: F, t730: F, t1957: F, t9397: F, t3626: F, t25588: F, t25590: F, t25592: F, t25596: F, t25601: F, t25603: F, t25606: F) -> (F, F, F, F, F, F) {
    let t25609 = 0.2069040516770936012e4 * t20659 * t20918 * t683;
    let t25611 = 0.11696447245269292414e1 * t5754 * t3618;
    let t25614 = 0.35089341735807877242e1 * t730 * t3625 * t1972;
    let t25617 = 0.14035736694323150897e2 * t730 * t9397 * t1957;
    let t25619 = 0.17315859105681463759e2 * t5754 * t3626;
    let t25620 = -t25588 - t25590 - t25592 + t25596 + t25601 + t25603 - t25606 + t25609 + t25611 - t25614 + t25617 - t25619;
    (t25609, t25611, t25614, t25617, t25619, t25620)
}
