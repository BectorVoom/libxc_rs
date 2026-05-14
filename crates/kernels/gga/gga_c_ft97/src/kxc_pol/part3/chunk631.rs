//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 631/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk631<F: Float>(t1882: F, t3463: F, t3480: F, t3485: F, t1045: F, t2178: F, t3584: F, t3580: F, t3571: F, t3442: F, t8392: F, t582: F, t167: F, t9132: F, t605: F, t2097: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t12620 = 2.0 / 27.0 * t1882 * t3463;
    let t12642 = 2.0 / 9.0 * t1882 * t3480;
    let t12644 = 4.0 / 9.0 * t1882 * t3485;
    let t12664 = t1045 * t2178;
    let t12670 = 2.0 / 9.0 * t1882 * t3584;
    let t12672 = 2.0 / 9.0 * t1882 * t3580;
    let t12674 = 2.0 / 9.0 * t1882 * t3571;
    let t12676 = 4.0 / 81.0 * t8392 * t3442;
    let t12680 = t582 * t1045;
    let t12703 = t9132 * t167;
    let t12709 = t582 * t605;
    let t12714 = t2097 * t605;
    (t12620, t12642, t12644, t12664, t12670, t12672, t12674, t12676, t12680, t12703, t12709, t12714)
}
