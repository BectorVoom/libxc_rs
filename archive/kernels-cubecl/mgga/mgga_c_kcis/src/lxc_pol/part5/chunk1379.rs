//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1379/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1379<F: Float>(t4249: F, t7283: F, t6002: F, t6034: F, t2066: F, t6020: F, t21955: F, t577: F, t1548: F, t1929: F, t570: F, t5910: F) -> (F, F, F, F, F) {
    let t22685 = t4249 * t7283;
    let t22687 = t6002 * t6034;
    let t22689 = t6020 * t2066;
    let t22691 = t21955 * t577;
    let t22692 = t22691 * t1548;
    let t22694 = t570 * t1929;
    let t22695 = t22694 * t5910;
    (t22685, t22687, t22689, t22692, t22695)
}
