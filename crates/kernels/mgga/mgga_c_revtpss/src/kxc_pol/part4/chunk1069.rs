//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1069/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1069<F: Float>(t1207: F, t456: F, t487: F, t1269: F, t3566: F, t1203: F, t3565: F, t3552: F, t1208: F, t3551: F, t1209: F, t3727: F) -> (F, F, F, F, F, F, F, F, F) {
    let t12625 = t1207 * t1207;
    let t12626 = F::cast_from(1.0_f64) / t12625;
    let t12627 = t456 * t12626;
    let t12628 = t12627 * t487;
    let t12633 = t3566 * t1269;
    let t12640 = t1203 * t3565;
    let t12641 = t12640 * t487;
    let t12654 = t3552 * t487;
    let t12657 = t3551 * t1208;
    let t12658 = t12657 * t487;
    let t12666 = t1209 * t3727;
    (t12627, t12628, t12633, t12640, t12641, t12654, t12657, t12658, t12666)
}
