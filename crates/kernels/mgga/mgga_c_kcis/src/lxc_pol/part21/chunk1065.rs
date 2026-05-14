//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1065/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1065<F: Float>(t26446: F, t710: F, t86: F, t137: F, t8999: F, t125: F, t754: F, t8750: F, t2425: F, t7603: F, t26439: F, t748: F, t2480: F, t8955: F, t2490: F, t2526: F, t752: F, t774: F) -> (F, F, F, F, F, F, F, F, F) {
    let t91922 = t86 * t710 * t26446;
    let t91925 = t86 * t8999 * t137;
    let t91929 = t86 * t125 * t754 * t8750;
    let t91932 = t86 * t2425 * t7603;
    let t91935 = t86 * t748 * t26439;
    let t91938 = t86 * t748 * t26446;
    let t91941 = t86 * t2480 * t7603;
    let t91944 = t86 * t8955 * t137;
    let t91948 = t752 * t2490 * t2526 * t774;
    (t91922, t91925, t91929, t91932, t91935, t91938, t91941, t91944, t91948)
}
