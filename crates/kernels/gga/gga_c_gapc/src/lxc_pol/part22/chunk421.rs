//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 421/1209 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk421<F: Float>(t2059: F, t2370: F, t2078: F, t858: F, t10: F, t103: F, t164: F, t2150: F, t2346: F, t2350: F, t2356: F, t2358: F, t2362: F, t2364: F, t2367: F, t266: F, t303: F, t304: F, t758: F, t79: F, t849: F, t853: F, t855: F, t859: F) -> (F,) {
    let t2371 = t2370 * t2059;
    let t2374 = t858 * t2078;
    let t2386 = 0.58998125e-2 * t2346 * t304 - 0.2359925e-1 * t2350 * t855 - 0.11799625e-1 * t849 * t859 + 0.19666041666666666667e-2 * t2356 * t2358 + 0.2359925e-1 * t2362 * t2364 + 0.15732833333333333333e-1 * t853 * t2367 + 0.11799625e-1 * t303 * t2371 - 0.58998125e-2 * t303 * t2374 + 0.47803703703703703703e-2 * t103 * t79 * t266 - 0.28682222222222222222e-1 * t103 * t10 * t758 - 0.21511666666666666667e-1 * t103 * t164 * t2150;
    (t2386,)
}
