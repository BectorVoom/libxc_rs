//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 818/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk818<F: Float>(t558: F, t7407: F, t574: F, t605: F, t2178: F, t7400: F, t379: F, t2210: F, t1384: F, t5842: F, t1882: F, t7359: F) -> (F, F, F, F, F, F, F, F) {
    let t33050 = t7407 * t558;
    let t33052 = t574 * t605 * t33050;
    let t33055 = t2178 * t7400;
    let t33056 = t33055 * t379;
    let t33057 = t2210 * t33056;
    let t33060 = t5842 * t1384;
    let t33062 = t574 * t605 * t33060;
    let t33066 = F::new(2.0) / F::new(9.0) * t1882 * t7359;
    (t33050, t33052, t33055, t33056, t33057, t33060, t33062, t33066)
}
