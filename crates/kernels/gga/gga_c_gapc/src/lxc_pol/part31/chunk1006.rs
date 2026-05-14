//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1006/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1006<F: Float>(t11910: F, t29350: F, t125: F, t8448: F, t9059: F, t10293: F, t28524: F, t28526: F, t11356: F, t3363: F, t9865: F, t33211: F, t7595: F, t28602: F, t3784: F, t3131: F, t8785: F) -> (F, F, F, F, F, F, F) {
    let t33396 = t11910 * t29350;
    let t33399 = t9059 * t8448 * t125;
    let t33402 = t28524 * t33399 * t10293 * t28526;
    let t33405 = t3363 * t11356 * t9865;
    let t33407 = t33211 * t7595;
    let t33409 = t3784 * t28602;
    let t33411 = t3131 * t8785;
    (t33396, t33399, t33402, t33405, t33407, t33409, t33411)
}
