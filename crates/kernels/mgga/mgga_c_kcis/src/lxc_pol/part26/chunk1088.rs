//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1088/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1088<F: Float>(t26463: F, t91982: F, t26467: F, t874: F, t91985: F, t26470: F, t91978: F, t91972: F, t2157: F, t37041: F, t2720: F, t9194: F, t2398: F, t8939: F, t26459: F, t7639: F) -> (F, F, F, F, F, F, F, F) {
    let t91989 = t26463 * t91982;
    let t91992 = t874 * t91985 * t26467;
    let t91994 = t26470 * t91978;
    let t91996 = t26470 * t91972;
    let t91999 = t874 * t37041 * t2157;
    let t92002 = t2720 * t9194 * t2157;
    let t92005 = t8939 * t2398 * t2157;
    let t92007 = t26459 * t7639;
    (t91989, t91992, t91994, t91996, t91999, t92002, t92005, t92007)
}
