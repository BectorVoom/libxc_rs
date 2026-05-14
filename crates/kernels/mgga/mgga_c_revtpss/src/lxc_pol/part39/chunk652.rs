//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 652/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk652<F: Float>(t3153: F, t3154: F, t3152: F, t1042: F, t1036: F, t3148: F, t3141: F, t357: F, t1038: F, t1052: F, t1033: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t3155 = t3153 * t3154;
    let t3156 = t3152 * t3155;
    let t3157 = t1042 * t3156;
    let t3160 = t1036 * t3148;
    let t3161 = t3141 * t3160;
    let t3162 = t3153 * t357;
    let t3163 = t3152 * t3162;
    let t3164 = t1042 * t3163;
    let t3167 = t1052 * t1038;
    let t3168 = t1036 * t3167;
    let t3169 = t1033 * t3168;
    (t3155, t3156, t3157, t3160, t3161, t3162, t3163, t3164, t3168, t3169)
}
