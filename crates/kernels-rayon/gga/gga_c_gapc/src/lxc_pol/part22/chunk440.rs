//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 440/1426 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk440(t10: f64, t103: f64, t164: f64, t2150: f64, t2346: f64, t2350: f64, t2356: f64, t2358: f64, t2362: f64, t2364: f64, t2367: f64, t2371: f64, t2374: f64, t266: f64, t303: f64, t304: f64, t758: f64, t79: f64, t849: f64, t853: f64, t855: f64, t859: f64) -> f64 {
    let t2386 = 0.58998125e-2_f64 * t2346 * t304 - 0.2359925e-1_f64 * t2350 * t855 - 0.11799625e-1_f64 * t849 * t859 + 0.19666041666666666667e-2_f64 * t2356 * t2358 + 0.2359925e-1_f64 * t2362 * t2364 + 0.15732833333333333333e-1_f64 * t853 * t2367 + 0.11799625e-1_f64 * t303 * t2371 - 0.58998125e-2_f64 * t303 * t2374 + 0.47803703703703703703e-2_f64 * t103 * t79 * t266 - 0.28682222222222222222e-1_f64 * t103 * t10 * t758 - 0.21511666666666666667e-1_f64 * t103 * t164 * t2150;
    t2386
}
