//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1143/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1143<F: Float>(t2157: F, t2720: F, t9194: F, t2398: F, t8939: F, t26459: F, t7639: F, t36533: F, t695: F, t26477: F, t7642: F, t209: F, t213: F, t36902: F, t8762: F) -> (F, F, F, F, F, F) {
    let t92002 = t2720 * t9194 * t2157;
    let t92005 = t8939 * t2398 * t2157;
    let t92007 = t26459 * t7639;
    let t92010 = t36533 * t695 * t7639;
    let t92012 = t7642 * t26477;
    let t92016 = t209 * t213 * t36902 * t8762;
    (t92002, t92005, t92007, t92010, t92012, t92016)
}
