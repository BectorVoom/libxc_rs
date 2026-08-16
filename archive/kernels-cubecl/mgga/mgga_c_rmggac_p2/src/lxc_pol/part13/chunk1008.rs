//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1008/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1008<F: Float>(t35523: F, t9222: F, t36733: F, t8450: F, t7478: F, t1970: F, t209: F, t236: F, t40433: F, t7231: F, t7255: F, t9165: F) -> (F, F, F, F) {
    let t42083 = t9222 * t35523;
    let t42085 = t8450 * t36733;
    let t42086 = t42085 * t7478;
    let t42091 = t1970 * t7231 * t236 * t40433 * t209;
    let t42093 = t7255 * t9165;
    (t42083, t42086, t42091, t42093)
}
