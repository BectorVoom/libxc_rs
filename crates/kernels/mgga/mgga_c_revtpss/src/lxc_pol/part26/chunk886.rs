//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 886/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk886<F: Float>(t1211: F, t12621: F, t1207: F, t456: F, t487: F, t1214: F, t3568: F, t1269: F, t3566: F, t1203: F, t3565: F, t3584: F, t3790: F, t1277: F, t3552: F, t1208: F, t3551: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t12622 = t1211 * t12621;
    let t12625 = t1207 * t1207;
    let t12626 = 1.0 / t12625;
    let t12627 = t456 * t12626;
    let t12628 = t12627 * t487;
    let t12629 = t3568 * t1214;
    let t12630 = t1211 * t12629;
    let t12633 = t3566 * t1269;
    let t12640 = t1203 * t3565;
    let t12641 = t12640 * t487;
    let t12646 = t1214 * t3584;
    let t12647 = t1211 * t12646;
    let t12650 = t1214 * t3790;
    let t12651 = t1277 * t12650;
    let t12654 = t3552 * t487;
    let t12657 = t3551 * t1208;
    (t12622, t12627, t12628, t12629, t12630, t12633, t12640, t12641, t12646, t12647, t12651, t12654, t12657)
}
