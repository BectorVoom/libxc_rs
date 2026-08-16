//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1181/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1181<F: Float>(t3960: F, t5623: F, t1494: F, t5627: F, t3977: F, t498: F, t12133: F, t16848: F, t12159: F, t613: F, t1380: F, t1938: F) -> (F, F, F, F, F, F, F) {
    let t51799 = t5623 * t3960;
    let t52073 = t1494 * t5627;
    let t52460 = t3977 * t498;
    let t52613 = t12133 * t498;
    let t52649 = t16848 * t498;
    let t52696 = t613 * t12159;
    let t52697 = t1938 * t1380;
    (t51799, t52073, t52460, t52613, t52649, t52696, t52697)
}
