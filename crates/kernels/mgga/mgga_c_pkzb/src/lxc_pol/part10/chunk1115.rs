//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1115/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1115<F: Float>(t10066: F, t2381: F, t3199: F, t394: F, t3186: F, t406: F, t3874: F, t5728: F) -> (F, F, F, F, F) {
    let t10067 = t2381 * t10066;
    let t10070 = t394 * t3199;
    let t10071 = t3186 * t10070;
    let t10072 = t406 * t10071;
    let t10075 = t3874 * t5728;
    (t10067, t10070, t10071, t10072, t10075)
}
