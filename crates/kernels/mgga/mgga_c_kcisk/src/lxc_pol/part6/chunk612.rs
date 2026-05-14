//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 612/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk612<F: Float>(t1974: F, t9124: F, t5400: F, t9108: F, t1685: F, t8590: F, t8607: F, t4790: F, t1966: F, t1979: F, t2605: F, t2609: F, t5373: F, t5398: F, t5408: F, t5415: F, t7467: F, t7498: F, t764: F, t8546: F, t8548: F, t8552: F, t8576: F, t8579: F, t8585: F, t9103: F, t9109: F) -> (F, F, F, F, F, F) {
    let t9125 = t9124 * t1974;
    let t9128 = t9108 * t5400;
    let t9134 = t8590 * t1685;
    let t9137 = t8607 * t1685;
    let t9140 = t8590 * t4790;
    let t9143 = -0.3109e-1 * t9103 * t764 + 2.0 * t7467 * t2605 - 2.0 * t5373 * t9109 + 1.0 * t1966 * t9125 + 0.32164683177870697974e2 * t5398 * t9128 + t8546 - t8548 + t8552 - t8576 - t8579 - 0.19751789702565206229e-1 * t8585 + 0.11696446794910408142e1 * t7498 * t2609 - 0.11696446794910408142e1 * t5408 * t9134 + 0.58482233974552040708e0 * t1979 * t9137 + 0.17315755899375863299e2 * t5415 * t9140;
    (t9125, t9128, t9134, t9137, t9140, t9143)
}
