//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2100/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2100<F: Float>(t17617: F, t26870: F, t3682: F, t8172: F, t29020: F, t3704: F, t29086: F, t3678: F, t3655: F, t8185: F, t17628: F, t7607: F) -> (F, F, F, F, F, F) {
    let t104953 = F::cast_from(0.57165357490759649296e-3_f64) * t26870 * t17617;
    let t104963 = t8172 * t3682;
    let t104968 = F::cast_from(0.30488190661738479624e-2_f64) * t29020 * t3704;
    let t104972 = F::cast_from(0.57165357490759649296e-3_f64) * t29086 * t3678;
    let t104988 = t8185 * t3655;
    let t104990 = t7607 * t17628;
    (t104953, t104963, t104968, t104972, t104988, t104990)
}
