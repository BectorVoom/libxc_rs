//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1031/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1031<F: Float>(t12295: F, t1207: F, t456: F, t487: F, t1269: F, t3566: F, t1203: F, t3565: F, t3552: F, t1208: F, t3551: F, t1209: F, t3727: F, t460: F, t1284: F, t1204: F, t3766: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t12610 = 0.46096296296296296297e-1 * t12295;
    let t12625 = t1207 * t1207;
    let t12626 = 1.0 / t12625;
    let t12627 = t456 * t12626;
    let t12628 = t12627 * t487;
    let t12633 = t3566 * t1269;
    let t12640 = t1203 * t3565;
    let t12641 = t12640 * t487;
    let t12654 = t3552 * t487;
    let t12657 = t3551 * t1208;
    let t12658 = t12657 * t487;
    let t12666 = t1209 * t3727;
    let t12673 = t460 * t3727;
    let t12678 = 0.25925925925925925926e-1 * t12295;
    let t12699 = t3552 * t1284;
    let t12702 = t1204 * t3766;
    (t12610, t12627, t12628, t12633, t12640, t12641, t12654, t12657, t12658, t12666, t12673, t12678, t12699, t12702)
}
