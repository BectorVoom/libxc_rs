//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 439/1429 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk439<F: Float>(t2316: F, t772: F, t268: F, t78: F, t760: F, t9: F, t2152: F, t22: F, t2250: F, t2254: F, t2299: F, t768: F) -> (F, F, F, F, F, F) {
    let t2317 = t772 * t2316;
    let t2320 = t78 * t268;
    let t2324 = t9 * t760;
    let t2331 = t22 * t2152;
    let t2338 = t2250 * t2254;
    let t2342 = t768 * t2299;
    (t2317, t2320, t2324, t2331, t2338, t2342)
}
