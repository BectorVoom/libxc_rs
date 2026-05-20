//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3525/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3525<F: Float>(t11921: F, t19399: F, t247: F, t4837: F, t15752: F, t19741: F, t3091: F, t43240: F, t6267: F, t16088: F, t380: F, t4746: F) -> (F, F, F, F) {
    let t66752 = t4837 * t247 * t11921 * t19399;
    let t66758 = t19741 * t15752;
    let t66763 = t3091 * t43240 * t6267;
    let t66766 = t4746 * t380 * t16088;
    (t66752, t66758, t66763, t66766)
}
