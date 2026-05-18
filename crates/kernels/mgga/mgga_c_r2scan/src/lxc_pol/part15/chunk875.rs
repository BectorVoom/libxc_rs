//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 875/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk875<F: Float>(t1554: F, t2124: F, t2550: F, t2294: F, t2583: F, t2582: F, t1551: F, t2572: F, t360: F, t113: F, t1234: F, t6063: F, t7605: F) -> (F, F, F, F, F, F, F, F) {
    let t7934 = t2124 * t2550 * t1554;
    let t7937 = t2294 * t2583;
    let t7939 = F::new(0.23115257973478049502e0) * t2582 * t7937;
    let t7940 = t2572 * t1551;
    let t7941 = t360 * t7940;
    let t7944 = t113 * t1234;
    let t7945 = t2572 * t7944;
    let t7946 = t360 * t7945;
    let t7949 = t6063 * t7605;
    (t7934, t7939, t7940, t7941, t7944, t7945, t7946, t7949)
}
