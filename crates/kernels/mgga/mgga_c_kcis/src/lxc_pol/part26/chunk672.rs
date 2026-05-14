//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 672/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk672<F: Float>(t1555: F, t7940: F, t2253: F, t4184: F, t4189: F, t1528: F, t573: F, t1532: F, t491: F) -> (F, F, F, F, F, F) {
    let t7941 = t7940 * t1555;
    let t7942 = t4184 * t2253;
    let t7943 = t2253 * t1555;
    let t7945 = 2.0 * t4189 * t7943;
    let t7946 = t1528 * t573;
    let t7948 = t1532 * t491;
    (t7941, t7942, t7943, t7945, t7946, t7948)
}
