//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 508/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk508<F: Float>(t3116: F, t721: F, t3115: F, t355: F, t839: F, t1060: F, t1059: F, t1068: F, t1072: F, t301: F, t21: F, t5: F, t1049: F, t1056: F, t137: F, t167: F) -> (F, F, F, F, F, F, F, F, F) {
    let t3117 = t3116 * t721;
    let t3118 = t3115 * t3117;
    let t3120 = t355 * t839;
    let t3121 = t3120 * t721;
    let t3122 = t1060 * t3121;
    let t3124 = t1068 * t1059;
    let t3125 = t1072 * t301;
    let t3126 = t21 * t5;
    let t3127 = t3125 * t3126;
    let t3128 = t3124 * t3127;
    let t3130 = t1049 * t1056;
    let t3132 = t167 * t137;
    (t3118, t3120, t3122, t3124, t3125, t3126, t3128, t3130, t3132)
}
