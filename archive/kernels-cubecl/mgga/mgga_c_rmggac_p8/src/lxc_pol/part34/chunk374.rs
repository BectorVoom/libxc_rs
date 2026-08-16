//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 374/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk374<F: Float>(t464: F, t483: F, t1968: F, t1966: F, t1004: F, t108: F, t490: F) -> (F, F, F) {
    let t7242 = t464 * t483;
    let t7243 = t7242 * t1968;
    let t7244 = t1966 * t7243;
    let t7247 = t1004 * t108;
    let t7248 = t490 * t7247;
    (t7244, t7247, t7248)
}
