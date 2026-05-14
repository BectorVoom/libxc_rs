//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 877/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk877<F: Float>(t2223: F, t5916: F, t12703: F, t379: F, t569: F, t5975: F, t1359: F, t2075: F, t167: F, t2185: F, t5860: F, t616: F, t558: F, t5842: F, t1882: F, t5862: F) -> (F, F, F, F, F, F, F, F, F) {
    let t23510 = t5916 * t2223;
    let t23511 = t12703 * t23510;
    let t23515 = t569 * t5975 * t379;
    let t23518 = t1359 * t2075;
    let t23520 = t2185 * t167 * t23518;
    let t23524 = t2185 * t616 * t5860;
    let t23527 = t5842 * t558;
    let t23529 = t2185 * t167 * t23527;
    let t23532 = t1882 * t5862;
    (t23510, t23511, t23515, t23518, t23520, t23524, t23527, t23529, t23532)
}
