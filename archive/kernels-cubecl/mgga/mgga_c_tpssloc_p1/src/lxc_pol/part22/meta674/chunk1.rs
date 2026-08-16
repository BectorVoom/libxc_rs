//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2232/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2232<F: Float>(t10224: F, t5828: F, t973: F, t42875: F, t5817: F, t17763: F, t2960: F, t18057: F, t225: F, t18059: F, t1020: F, t17960: F, t248: F, t3101: F) -> (F, F, F, F, F, F) {
    let t61597 = t973 * t10224 * t5828;
    let t61600 = t973 * t42875 * t5817;
    let t61602 = t2960 * t17763;
    let t61621 = t18057 * t225;
    let t61646 = t18059 * t225;
    let t61655 = t1020 * t248 * t3101 * t17960;
    (t61597, t61600, t61602, t61621, t61646, t61655)
}
