//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1364/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1364<F: Float>(t17412: F, t5919: F, t17391: F, t5916: F, t1534: F, t7385: F, t1533: F, t1529: F, t7389: F, t22212: F, t584: F, t583: F) -> (F, F, F, F, F) {
    let t22442 = t17412 * t5919;
    let t22444 = t17391 * t5916;
    let t22446 = t7385 * t1534;
    let t22447 = t1533 * t22446;
    let t22449 = t1529 * t7389;
    let t22451 = t584 * t22212;
    let t22452 = t583 * t22451;
    (t22442, t22444, t22447, t22449, t22452)
}
