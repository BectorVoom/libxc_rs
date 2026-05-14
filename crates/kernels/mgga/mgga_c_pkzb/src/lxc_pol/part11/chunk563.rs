//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 563/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk563<F: Float>(t1197: F, t870: F, t2175: F, t2224: F, t2264: F, t2269: F, t3017: F, t3028: F, t3042: F, t3047: F, t3053: F, t3055: F, t3059: F, t3063: F, t3067: F) -> (F, F) {
    let t3088 = t1197 * t870;
    let t3102 = -0.17648625e1 * t3042 + 0.3529725e1 * t3047 + t2264 - 0.516475e0 * t2175 - 0.516475e0 * t3017 + 0.1549425e1 * t3028 + 0.31558125e0 * t3053 + 0.6311625e0 * t3055 + t2269 - 0.20839e0 * t2224 - 0.20839e0 * t3059 + 0.312585e0 * t3063 + 0.312585e0 * t3067;
    (t3088, t3102)
}
