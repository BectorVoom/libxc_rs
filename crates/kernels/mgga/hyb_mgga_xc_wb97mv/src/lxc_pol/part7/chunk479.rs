//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 479/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk479<F: Float>(t778: F, t238: F, t242: F, t2187: F, t226: F, t2178: F, t2189: F, t2207: F, t2212: F, t2214: F, t2218: F, t2220: F, t2227: F, t2229: F) -> (F, F, F, F, F) {
    let t2231 = t778 * t778;
    let t2233 = t238 * t242 * t2231;
    let t2235 = t226 * t2187;
    let t2237 = t238 * t242 * t2235;
    let t2239 = -0.9494625e0 * t2207 + 0.1898925e1 * t2212 + t2214 - 0.59793333333333333334e0 * t2178 + 0.8969e0 * t2189 + 0.15358125e0 * t2218 + 0.3071625e0 * t2220 + t2227 - 0.32862666666666666666e0 * t2229 + 0.24647e0 * t2233 + 0.24647e0 * t2237;
    (t2231, t2233, t2235, t2237, t2239)
}
