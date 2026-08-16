//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2304/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2304<F: Float>(t3302: F, t357: F, t4982: F, t999: F, t1647: F, t4980: F, t4995: F, t1678: F, t3298: F, t342: F, t3316: F, t1045: F) -> (F, F, F, F, F, F, F, F, F) {
    let t19482 = t3302 * t357;
    let t19502 = t4982 * t999;
    let t19526 = t1647 * t4980;
    let t19569 = t1647 * t4995;
    let t19579 = t19482 * t999;
    let t19602 = t3298 * t1678;
    let t19603 = t342 * t19602;
    let t19607 = t3316 * t1678;
    let t19608 = t342 * t19607;
    let t19620 = t1045 * t999;
    (t19502, t19526, t19569, t19579, t19602, t19603, t19607, t19608, t19620)
}
