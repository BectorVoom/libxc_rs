//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 971/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk971<F: Float>(t2237: F, t27348: F, t3717: F, t531: F, t4142: F, t7925: F, t1542: F, t491: F) -> (F, F, F, F) {
    let t27349 = t2237 * t27348;
    let t27356 = t3717 * t531;
    let t27362 = t4142 * t7925;
    let t27364 = t1542 * t491;
    (t27349, t27356, t27362, t27364)
}
