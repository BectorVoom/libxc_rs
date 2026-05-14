//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 950/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk950<F: Float>(t2206: F, t2389: F, t2211: F, t2299: F, t2404: F, t2546: F, t122: F, t188: F, t311: F, t6851: F, t6: F, t6856: F, t2387: F, t2577: F, t2598: F, t286: F) -> (F, F, F, F, F, F, F, F) {
    let t18866 = t2389 * t2206;
    let t19048 = t2211 * t2299;
    let t19055 = t2546 * t2404;
    let t19094 = t311 * t6851 * t122 * t188;
    let t19097 = t6856 * t6;
    let t19120 = t2387 * t2577;
    let t19139 = t2598 * t2404;
    let t19159 = t2299 * t286;
    (t18866, t19048, t19055, t19094, t19097, t19120, t19139, t19159)
}
