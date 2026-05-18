//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1314/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1314<F: Float>(t2368: F, t824: F, t300: F, t3175: F, t3185: F, t8381: F, t926: F, t8423: F, t8428: F, t8431: F, t54: F, t8253: F) -> (F, F, F, F, F, F) {
    let t23167 = t2368 * t824;
    let t23176 = t300 * t3175;
    let t23201 = t3185 * t926 * t8381;
    let t23204 = t3185 * t926 * t8423;
    let t23207 = t8428 * t926 * t8431;
    let t23213 = t54 * t8253;
    (t23167, t23176, t23201, t23204, t23207, t23213)
}
