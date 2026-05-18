//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1226/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1226<F: Float>(t165: F, t92213: F, t92254: F, t92292: F, t92336: F, t2538: F, t2626: F, t7630: F, t26416: F, t826: F, t9275: F, t26398: F, t9279: F) -> (F, F, F, F) {
    let t92339 = (t92213 + t92254 + t92292 + t92336) * t165;
    let t92344 = F::new(6.0) * t2538 * t7630 * t2626;
    let t92351 = F::new(18.0) * t9275 * t26416 * t826;
    let t92356 = F::new(6.0) * t26398 * t9279;
    (t92339, t92344, t92351, t92356)
}
