//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 243/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk243<F: Float>(t1385: F, t1386: F, t1014: F, t558: F, t526: F) -> (F, F, F, F) {
    let t1387 = t1385 * t1386;
    let t1390 = t1014 * t558;
    let t1391 = F::cast_from(0.16581944444444444444e-2_f64) * t1390;
    let t1392 = F::new(1.0) / t526;
    (t1387, t1390, t1391, t1392)
}
