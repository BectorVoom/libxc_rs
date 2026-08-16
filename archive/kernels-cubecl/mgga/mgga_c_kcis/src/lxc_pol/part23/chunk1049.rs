//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1049/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1049<F: Float>(t1014: F, t7928: F, t2243: F, t4109: F, t303: F, t27348: F, t7898: F, t1458: F, t1466: F, t1490: F, t1459: F, t1498: F) -> (F, F, F, F, F, F, F, F) {
    let t27462 = t1014 * t7928;
    let t27464 = t4109 * t2243;
    let t27465 = t303 * t27464;
    let t27471 = t7898 * t27348;
    let t27475 = t1458 * t1466;
    let t27476 = t27475 * t1490;
    let t27477 = t303 * t27476;
    let t27479 = t1459 * t1498;
    (t27462, t27464, t27465, t27471, t27475, t27476, t27477, t27479)
}
