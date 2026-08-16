//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1840/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1840<F: Float>(t1888: F, t25045: F, t82159: F, t6562: F, t7488: F, t82133: F, t25225: F, t6547: F, t23168: F, t25338: F, t23012: F, t7485: F) -> (F, F, F, F, F) {
    let t86933 = t1888 * t82159 * t25045;
    let t86940 = t6562 * t82133 * t7488;
    let t86942 = t6547 * t25225;
    let t86950 = t23168 * t25338;
    let t86955 = t23012 * t7485;
    (t86933, t86940, t86942, t86950, t86955)
}
