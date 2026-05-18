//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 1013/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk1013<F: Float>(t3712: F, t5625: F, t137: F, t1403: F, t442: F, t5215: F, t1: F, t5700: F, t19508: F, t4867: F, t144: F, t5698: F) -> (F, F, F, F, F, F) {
    let t19546 = t3712 * t5625;
    let t19586 = t1403 * t137;
    let t19588 = t5215 * t19586 * t442;
    let t19622 = t5700 * t1;
    let t19624 = t19508 * t19622 * t4867;
    let t19636 = t144 * t5698;
    (t19546, t19586, t19588, t19622, t19624, t19636)
}
