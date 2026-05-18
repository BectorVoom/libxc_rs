//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1121/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1121<F: Float>(t179: F, t19155: F, t2226: F, t404: F, t154: F, t385: F, t386: F, t4932: F, t6185: F, t921: F, t466: F, t931: F) -> (F, F, F, F) {
    let t19158 = t404 * t179 * t19155 * t2226;
    let t19163 = F::new(5.0) / F::new(486.0) * t385 * t154 * t4932 * t386;
    let t19166 = t921 * t6185;
    let t19191 = t466 * t931;
    (t19158, t19163, t19166, t19191)
}
