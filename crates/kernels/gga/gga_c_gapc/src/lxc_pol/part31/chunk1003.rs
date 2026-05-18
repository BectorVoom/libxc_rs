//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1003/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1003<F: Float>(t11513: F, t1743: F, t1749: F, t190: F, t632: F, t11449: F, t11451: F, t5117: F, t1: F, t8820: F) -> (F, F, F, F, F, F, F) {
    let t11514 = t1743 * t11513;
    let t11515 = t11514 * t1749;
    let t11517 = t632 * t190;
    let t11518 = t11517 * t11449;
    let t11519 = t11451 * t5117;
    let t11520 = t11518 * t11519;
    let t11522 = t8820 * t1;
    (t11514, t11515, t11517, t11518, t11519, t11520, t11522)
}
