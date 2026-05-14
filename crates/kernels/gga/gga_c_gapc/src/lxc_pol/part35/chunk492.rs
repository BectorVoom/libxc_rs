//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 492/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk492<F: Float>(t1938: F, t599: F, t596: F, t1936: F, t611: F, t1894: F, t618: F, t646: F, t1026: F, t633: F, t637: F, t670: F, t87: F, t8: F, t1734: F, t1743: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t3031 = t1938 * t599;
    let t3032 = t596 * t3031;
    let t3034 = t611 * t1936;
    let t3035 = t618 * t1894;
    let t3036 = t646 * t3035;
    let t3037 = t3034 * t3036;
    let t3039 = t633 * t1026;
    let t3040 = t3039 * t637;
    let t3042 = t670 * t87;
    let t3044 = 1.0 / t8 / t3042;
    let t3045 = t1734 * t3044;
    let t3046 = t1743 * t3045;
    (t3031, t3032, t3034, t3036, t3037, t3039, t3040, t3042, t3044, t3045, t3046)
}
