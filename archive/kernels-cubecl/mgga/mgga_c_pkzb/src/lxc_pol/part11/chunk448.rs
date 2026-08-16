//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 448/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk448<F: Float>(t154: F, t2048: F, t655: F, t276: F, t735: F, t739: F, t301: F, t466: F, t178: F, t299: F, t53: F, t779: F) -> (F, F, F, F, F, F) {
    let t2050 = t154 * t2048 * t655;
    let t2051 = t276 * t2050;
    let t2060 = t735 * t739;
    let t2064 = t466 * t301;
    let t2065 = t178 * t2064;
    let t2067 = F::cast_from(0.47637797908966374413e-4_f64) * t299 * t2065;
    let t2068 = t53 * t779;
    (t2051, t2060, t2064, t2065, t2067, t2068)
}
