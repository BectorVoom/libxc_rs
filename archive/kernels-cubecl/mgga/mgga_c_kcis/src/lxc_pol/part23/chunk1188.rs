//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1188/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1188<F: Float>(t18210: F, t27415: F, t7898: F, t2237: F, t11425: F, t1386: F, t94491: F, t94469: F, t1466: F, t4109: F, t3245: F, t7932: F) -> (F, F, F, F, F, F, F) {
    let t94496 = t18210 * t27415;
    let t94497 = t7898 * t94496;
    let t94499 = t2237 * t94496;
    let t94519 = t1386 * t11425;
    let t94524 = t7898 * t94491;
    let t94526 = t7898 * t94469;
    let t94528 = t4109 * t1466;
    let t94537 = t3245 * t7932;
    (t94497, t94499, t94519, t94524, t94526, t94528, t94537)
}
