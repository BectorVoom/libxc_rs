//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 973/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk973<F: Float>(t2453: F, t9792: F, t240: F, t2712: F, t3994: F, t2713: F, t3951: F, t3964: F, t785: F, t9731: F, t225: F, t4062: F) -> (F, F, F, F, F, F) {
    let t9793 = t2453 * t9792;
    let t9794 = t2712 * t240;
    let t9795 = t9794 * t3994;
    let t9796 = t9793 * t9795;
    let t9799 = t3964 * t2713 * t3951;
    let t9801 = t9731 * t785;
    let t9802 = t9801 * t225;
    let t9804 = F::new(0.45738002528356795401e-4) * t9802 * t4062;
    (t9793, t9794, t9796, t9799, t9802, t9804)
}
