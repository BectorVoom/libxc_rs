//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 935/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk935<F: Float>(t2237: F, t27348: F, t1380: F, t833: F, t7909: F, t3984: F, t3717: F, t531: F, t1385: F) -> (F, F, F, F, F, F) {
    let t27349 = t2237 * t27348;
    let t27351 = t833 * t1380;
    let t27352 = t7909 * t27351;
    let t27353 = t3984 * t27352;
    let t27356 = t3717 * t531;
    let t27357 = t833 * t1385;
    (t27349, t27351, t27352, t27353, t27356, t27357)
}
