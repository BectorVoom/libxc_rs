//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1321/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1321<F: Float>(t1014: F, t29380: F, t2002: F, t303: F, t98607: F, t29386: F, t28524: F, t5633: F, t1983: F, t5757: F, t576: F, t7052: F) -> (F, F, F, F, F, F) {
    let t102649 = t1014 * t29380;
    let t102653 = t303 * t98607 * t2002;
    let t102655 = t1014 * t29386;
    let t102658 = t303 * t28524 * t5633;
    let t102661 = t303 * t1983 * t5757;
    let t102664 = t576 * t7052;
    (t102649, t102653, t102655, t102658, t102661, t102664)
}
