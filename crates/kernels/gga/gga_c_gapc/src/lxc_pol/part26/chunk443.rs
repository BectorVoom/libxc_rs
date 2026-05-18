//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 443/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk443<F: Float>(t2405: F, t772: F, t468: F, t820: F, t2158: F, t276: F, t653: F, t902: F, t128: F, t291: F) -> (F, F, F, F) {
    let t2406 = t772 * t2405;
    let t2409 = t468 * t820;
    let t2412 = t2158 * t276;
    let t2413 = t902 * t653;
    let t2414 = t2412 * t2413;
    let t2415 = t128 * t291;
    (t2406, t2409, t2414, t2415)
}
