//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 953/1209 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk953<F: Float>(t137: F, t1403: F, t442: F, t5215: F, t1: F, t5700: F, t19508: F, t4867: F, t144: F, t5698: F, t203: F, t9078: F, t19507: F, t4017: F, t681: F, t1266: F, t186: F) -> (F, F, F, F, F, F, F) {
    let t19586 = t1403 * t137;
    let t19588 = t5215 * t19586 * t442;
    let t19622 = t5700 * t1;
    let t19624 = t19508 * t19622 * t4867;
    let t19636 = t144 * t5698;
    let t19639 = t19636 * t203 * t19622 * t9078;
    let t19644 = t19507 * t681 * t19622 * t4017;
    let t19652 = t1266 * t186;
    (t19586, t19588, t19624, t19636, t19639, t19644, t19652)
}
