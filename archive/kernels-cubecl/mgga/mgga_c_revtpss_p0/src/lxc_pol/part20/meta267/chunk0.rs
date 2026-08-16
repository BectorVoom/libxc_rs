//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1115/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1115<F: Float>(t3154: F, t905: F, t606: F, t11659: F, t3092: F, t3095: F, t1052: F, t360: F, t3089: F) -> (F, F, F, F, F, F, F, F) {
    let t11660 = t3154 * t905;
    let t11661 = t11660 * t606;
    let t11662 = t11659 * t11661;
    let t11663 = t3092 * t11662;
    let t11666 = t11659 * t3095;
    let t11667 = t3092 * t11666;
    let t11670 = t360 * t1052;
    let t11671 = t11670 * t3089;
    (t11660, t11661, t11662, t11663, t11666, t11667, t11670, t11671)
}
