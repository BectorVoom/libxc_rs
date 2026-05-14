//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 419/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk419<F: Float>(t2316: F, t772: F, t268: F, t78: F, t760: F, t9: F, t2152: F, t22: F, t2250: F, t2254: F, t2299: F, t768: F, t159: F, t2141: F, t104: F, t751: F) -> (F, F, F, F, F, F, F, F) {
    let t2317 = t772 * t2316;
    let t2320 = t78 * t268;
    let t2324 = t9 * t760;
    let t2331 = t22 * t2152;
    let t2338 = t2250 * t2254;
    let t2342 = t768 * t2299;
    let t2346 = t2141 * t159;
    let t2349 = t751 * t104;
    (t2317, t2320, t2324, t2331, t2338, t2342, t2346, t2349)
}
