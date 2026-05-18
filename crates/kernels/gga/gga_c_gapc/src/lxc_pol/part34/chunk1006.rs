//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1006/1427 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1006<F: Float>(t11597: F, t2993: F, t3001: F, t1030: F, t3008: F, t11356: F, t9256: F, t932: F, t996: F, t3723: F, t787: F, t876: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t11598 = t2993 * t11597;
    let t11599 = t11598 * t3001;
    let t11601 = t1030 * t11597;
    let t11602 = t11601 * t3008;
    let t11604 = t2993 * t11356;
    let t11605 = t11604 * t9256;
    let t11612 = t996 * t932;
    let t11613 = t3723 * t787;
    let t11614 = t11612 * t11613;
    let t11616 = t3723 * t876;
    (t11598, t11599, t11601, t11602, t11604, t11605, t11612, t11613, t11614, t11616)
}
