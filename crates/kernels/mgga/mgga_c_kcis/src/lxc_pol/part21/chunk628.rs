//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 628/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk628<F: Float>(t1267: F, t5281: F, t1846: F, t3500: F, t1251: F, t2888: F, t421: F) -> (F, F, F, F) {
    let t5282 = t5281 * t1267;
    let t5299 = t3500 * t1846;
    let t5300 = t1251 * t5299;
    let t5302 = t2888 * t421;
    (t5282, t5299, t5300, t5302)
}
