//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 499/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk499<F: Float>(t2175: F, t2226: F, t2178: F, t2189: F, t2207: F, t2212: F, t2218: F, t2220: F, t2229: F, t2233: F, t2237: F) -> (F, F, F) {
    let t2308 = 0.40256666666666666667e0 * t2175;
    let t2313 = 0.137975e0 * t2226;
    let t2317 = -0.1294625e1 * t2207 + 0.258925e1 * t2212 + t2308 - 0.60385e0 * t2178 + 0.905775e0 * t2189 + 0.82524375e-1 * t2218 + 0.16504875e0 * t2220 + t2313 - 0.33114e0 * t2229 + 0.248355e0 * t2233 + 0.248355e0 * t2237;
    (t2308, t2313, t2317)
}
