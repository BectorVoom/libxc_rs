//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 500/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk500<F: Float>(t7: F, t2159: F, t220: F, t2337: F, t291: F, t771: F, t861: F, t909: F, t314: F, t1832: F, t319: F, t98: F, t322: F, t317: F, t99: F, t324: F, t295: F, t894: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F) {
    let t8 = t7 <= zeta_threshold;
    let t9 = rho0 <= dens_threshold || t8;
    let t2341 = piecewise3(t9, 0.0, t2159 * t291 / 2.0 + t771 * t861 + t220 * t2337 / 2.0);
    let t2345 = 1.0 / t909;
    let t2350 = t314 * t314;
    let t2351 = t319 * t1832;
    let t2353 = 1.0 / t98 / t2351;
    let t2355 = t322 * t322;
    let t2356 = 1.0 / t2355;
    let t2360 = t319 * t317;
    let t2362 = 1.0 / t99 / t2360;
    let t2372 = t2345 * t324;
    let t2375 = t295 * t894;
    (t2341, t2345, t2350, t2353, t2356, t2362, t2372, t2375)
}
