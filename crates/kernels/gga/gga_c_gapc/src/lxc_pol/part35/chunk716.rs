//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 716/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk716<F: Float>(t3949: F, t436: F, t8459: F, t1476: F, t2945: F, t126: F, t505: F, t568: F, t120: F, t152: F, t493: F, t5918: F) -> (F, F, F, F, F) {
    let t8460 = t436 * t3949;
    let t8461 = t8459 * t8460;
    let t8463 = t1476 * t2945;
    let t8465 = t126 * t505;
    let t8466 = t8465 * t568;
    let t8467 = t120 * t8466;
    let t8469 = t493 * t152;
    let t8470 = t8469 * t5918;
    (t8461, t8463, t8465, t8467, t8470)
}
