//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 451/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk451<F: Float>(t1624: F, t236: F, t1627: F, t511: F, t515: F, t8377: F, t495: F, t558: F, t109: F, t4179: F, t490: F, t498: F, t618: F) -> (F, F, F, F, F, F, F) {
    let t9189 = t236 * t1624;
    let t9193 = t511 * t1627;
    let t9197 = t515 * t8377;
    let t9205 = t511 * t558 * t495;
    let t9209 = t4179 * t109;
    let t9210 = t490 * t9209;
    let t9211 = t618 * t498;
    (t9189, t9193, t9197, t9205, t9209, t9210, t9211)
}
