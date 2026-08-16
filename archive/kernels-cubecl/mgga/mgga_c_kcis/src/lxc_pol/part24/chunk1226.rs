//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1226/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1226<F: Float>(t20129: F, t377: F, t8069: F, t95416: F, t1096: F, t20173: F, t95326: F, t4999: F, t5086: F, t19960: F, t28029: F, t19112: F, t5047: F, t7748: F) -> (F, F, F, F, F, F, F) {
    let t99984 = t20129 * t377;
    let t99986 = t95416 * t8069;
    let t99988 = t1096 * t20173;
    let t99990 = t95326 * t8069;
    let t99992 = t4999 * t5086;
    let t99994 = t28029 * t19960;
    let t99997 = t7748 * t5047 * t19112;
    (t99984, t99986, t99988, t99990, t99992, t99994, t99997)
}
