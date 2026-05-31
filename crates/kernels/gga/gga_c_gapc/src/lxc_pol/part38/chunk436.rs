//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 436/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk436<F: Float>(t10: F, t103: F, t164: F, t2150: F, t2346: F, t2350: F, t2356: F, t2358: F, t2362: F, t2364: F, t2367: F, t2371: F, t2374: F, t266: F, t303: F, t304: F, t758: F, t79: F, t849: F, t853: F, t855: F, t859: F) -> F {
    let t2386 = F::cast_from(0.58998125e-2_f64) * t2346 * t304 - F::cast_from(0.2359925e-1_f64) * t2350 * t855 - F::cast_from(0.11799625e-1_f64) * t849 * t859 + F::cast_from(0.19666041666666666667e-2_f64) * t2356 * t2358 + F::cast_from(0.2359925e-1_f64) * t2362 * t2364 + F::cast_from(0.15732833333333333333e-1_f64) * t853 * t2367 + F::cast_from(0.11799625e-1_f64) * t303 * t2371 - F::cast_from(0.58998125e-2_f64) * t303 * t2374 + F::cast_from(0.47803703703703703703e-2_f64) * t103 * t79 * t266 - F::cast_from(0.28682222222222222222e-1_f64) * t103 * t10 * t758 - F::cast_from(0.21511666666666666667e-1_f64) * t103 * t164 * t2150;
    t2386
}
