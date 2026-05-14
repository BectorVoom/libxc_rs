//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1016/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1016<F: Float>(t9552: F, t9559: F, t1317: F, t5567: F, t9564: F, t9566: F, t9578: F, t9580: F, t4147: F, t5778: F, t2496: F, t5571: F, t5569: F, t3829: F, t566: F, t1448: F, t1868: F, t198: F, t4139: F, t4140: F, t5541: F, t5591: F, t9514: F, t9517: F, t9521: F, t9555: F, t9569: F, t9574: F, t9577: F, t9588: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t13640 = 0.5848223622634646207e0 * t9552;
    let t13641 = 40.0 * t9559;
    let t13643 = 8.0 * t1317 * t5567;
    let t13644 = 0.18311447306006545054e-3 * t9564;
    let t13645 = 0.4883052614935078681e-3 * t9566;
    let t13646 = 24.0 * t9578;
    let t13647 = 4.0 * t9580;
    let t13648 = t5778 * t4147;
    let t13652 = t5571 * t2496;
    let t13653 = 0.17315859105681463759e2 * t13652;
    let t13654 = t1317 * t5569;
    let t13655 = 8.0 * t13654;
    let t13656 = t3829 * t566;
    let t13663 = -2.0 * t13648 * t1448 * t5541 + 6.0 * t13656 * t1868 * t198 + 6.0 * t4139 * t4140 * t5591 - t13640 + t13641 + t13643 - t13644 + t13645 - t13646 - t13647 - t13653 + t13655 + t9514 - t9517 - t9521 + t9555 + t9569 - t9574 - t9577 - t9588;
    (t13640, t13641, t13643, t13644, t13645, t13646, t13647, t13653, t13655, t13663)
}
