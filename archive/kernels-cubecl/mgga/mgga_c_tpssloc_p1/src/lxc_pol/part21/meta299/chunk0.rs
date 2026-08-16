//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1627/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1627<F: Float>(t1057: F, t10960: F, t3120: F, t3188: F, t10471: F, t10474: F, t10470: F) -> (F, F, F, F) {
    let t11051 = t10960 * t1057;
    let t11054 = t3188 * t3120;
    let t11058 = t10471 * t10474;
    let t11059 = t10470 * t11058;
    (t11051, t11054, t11058, t11059)
}
