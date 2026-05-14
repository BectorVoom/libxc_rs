//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 797/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk797<F: Float>(t1396: F, t5627: F, t1468: F, t1464: F, t1928: F, t556: F) -> (F, F, F, F) {
    let t5628 = t1396 * t5627;
    let t5629 = t1468 * t5628;
    let t5630 = t1464 * t5629;
    let t5632 = t1928 * t556;
    (t5628, t5629, t5630, t5632)
}
