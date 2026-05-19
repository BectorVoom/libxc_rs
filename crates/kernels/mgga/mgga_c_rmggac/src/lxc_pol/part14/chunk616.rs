//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 616/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk616<F: Float>(t678: F, t7921: F, t1550: F, t7810: F, t2084: F, t271: F) -> (F, F, F) {
    let t7922 = t7921 * t678;
    let t7924 = t1550 * t7810;
    let t7925 = F::cast_from(0.2993560425465952141e-1_f64) * t7924;
    let t7926 = t2084 * t271;
    (t7922, t7925, t7926)
}
