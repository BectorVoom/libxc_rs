//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 879/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk879<F: Float>(t11659: F, t11661: F, t3092: F, t3095: F, t1052: F, t360: F, t3089: F, t1087: F, t3090: F, t3278: F, t3133: F, t73: F, t2858: F, t4786: F, t3153: F, t4894: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t11662 = t11659 * t11661;
    let t11663 = t3092 * t11662;
    let t11666 = t11659 * t3095;
    let t11667 = t3092 * t11666;
    let t11670 = t360 * t1052;
    let t11671 = t11670 * t3089;
    let t11672 = t1087 * t11671;
    let t11675 = t3278 * t3090;
    let t11678 = t3133 * t73;
    let t11679 = t11678 * t3095;
    let t11680 = t3092 * t11679;
    let t11683 = t2858 * t4786;
    let t11684 = t3092 * t11683;
    let t11687 = t3133 * t3153;
    let t11688 = t11687 * t4894;
    (t11663, t11667, t11671, t11672, t11675, t11678, t11680, t11684, t11687, t11688)
}
