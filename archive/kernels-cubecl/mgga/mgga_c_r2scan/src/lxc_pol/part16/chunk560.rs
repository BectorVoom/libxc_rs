//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 560/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk560<F: Float>(t1568: F, t3071: F, t529: F, t551: F, t552: F, t3053: F, t538: F, t910: F, t938: F) -> (F, F, F, F, F, F, F) {
    let t3072 = t1568 * t3071;
    let t3073 = t529 * t3072;
    let t3077 = t551 * t552 * t3071;
    let t3081 = t551 * t552 * t3053;
    let t3086 = t538 * t3053;
    let t3087 = t529 * t3086;
    let t3090 = t938 * t910;
    (t3072, t3073, t3077, t3081, t3086, t3087, t3090)
}
