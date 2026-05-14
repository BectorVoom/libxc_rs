//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 293/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk293<F: Float>(t1113: F, t384: F, t336: F, t337: F, t839: F, t372: F, t429: F, t1047: F, t1053: F, t1075: F, t1050: F, t1057: F, t1063: F, t1066: F) -> (F, F, F, F, F, F, F) {
    let t1114 = t384 * t1113;
    let t1117 = t336 * t337 * t839;
    let t1121 = t336 * t429 * t372;
    let t1124 = 0.1141e1 * t1047;
    let t1126 = 0.2445e0 * t1053;
    let t1130 = 0.12225e0 * t1075;
    let t1131 = t1124 + 0.978e0 * t1050 - t1126 + 0.7335e0 * t1057 - 0.12225e0 * t1063 - 0.36675e0 * t1066 + t1130;
    (t1114, t1117, t1121, t1124, t1126, t1130, t1131)
}
