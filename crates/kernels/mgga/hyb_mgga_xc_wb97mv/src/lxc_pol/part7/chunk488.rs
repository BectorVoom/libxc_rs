//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 488/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk488<F: Float>(t2263: F, t828: F, t2175: F, t2226: F, t2178: F, t2189: F, t2207: F, t2212: F, t2218: F, t2220: F, t2229: F, t2233: F, t2237: F) -> (F, F, F, F) {
    let t2264 = t2263 * t828;
    let t2269 = 0.68863333333333333333e0 * t2175;
    let t2274 = 0.17365833333333333333e0 * t2226;
    let t2278 = -0.17648625e1 * t2207 + 0.3529725e1 * t2212 + t2269 - 0.103295e1 * t2178 + 0.1549425e1 * t2189 + 0.31558125e0 * t2218 + 0.6311625e0 * t2220 + t2274 - 0.41678e0 * t2229 + 0.312585e0 * t2233 + 0.312585e0 * t2237;
    (t2264, t2269, t2274, t2278)
}
