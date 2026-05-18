//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1077/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1077<F: Float>(t188: F, t4758: F, t6680: F, t13053: F, t732: F, t13113: F, t1983: F, t4743: F, t1320: F, t9534: F, t12997: F, t2229: F, t4744: F) -> (F, F, F, F, F, F, F) {
    let t37228 = t188 * t6680 * t4758;
    let t37258 = t732 * t13053;
    let t37294 = t13113 * t1983;
    let t37325 = t188 * t6680 * t4743;
    let t37328 = t9534 * t1320;
    let t37341 = t732 * t12997;
    let t37417 = t2229 * t4744;
    (t37228, t37258, t37294, t37325, t37328, t37341, t37417)
}
