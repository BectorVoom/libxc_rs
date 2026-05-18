//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 721/1426 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk721<F: Float>(t2953: F, t8337: F, t1004: F, t1265: F, t517: F, t1007: F, t2933: F, t2948: F, t2951: F, t423: F, t1459: F, t2954: F) -> (F, F, F, F, F, F) {
    let t8338 = t2953 * t8337;
    let t8340 = t1004 * t1265;
    let t8341 = t8340 * t517;
    let t8342 = t8341 * t1007;
    let t8344 = t2933 * t2948;
    let t8346 = t2951 * t423;
    let t8347 = t8346 * t1459;
    let t8348 = t8347 * t2954;
    (t8338, t8341, t8342, t8344, t8347, t8348)
}
