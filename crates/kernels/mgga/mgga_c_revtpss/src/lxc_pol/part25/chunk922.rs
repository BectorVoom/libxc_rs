//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 922/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk922<F: Float>(t11132: F, t11134: F, t11136: F, t11138: F, t11140: F, t11147: F, t11153: F, t11158: F, t11162: F, t11167: F, t11171: F, t341: F, t225: F, t366: F, t1053: F, t3196: F) -> (F, F, F, F, F) {
    let t11890 = 0.25925925925925925926e-1 * t11132;
    let t11901 = -t11890 - 0.11111111111111111111e-1 * t11134 + 0.55555555555555555555e-2 * t11136 - 0.16666666666666666667e-1 * t11138 + 0.83333333333333333334e-2 * t11140 - 0.92592592592592592592e-2 * t11147 + 0.33333333333333333333e-1 * t11153 - 0.16666666666666666666e-1 * t11158 - 0.50000000000000000001e-1 * t11162 + 0.50000000000000000001e-1 * t11167 - 0.83333333333333333333e-2 * t11171;
    let t11902 = t11901 * t341;
    let t11903 = t11902 * t225;
    let t11904 = t11903 * t366;
    let t11907 = t3196 * t1053;
    (t11901, t11902, t11903, t11904, t11907)
}
