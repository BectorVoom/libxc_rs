//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 939/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk939<F: Float>(t1084: F, t33158: F, t3415: F, t11784: F, t9865: F, t11379: F, t11945: F, t28594: F, t11948: F, t30095: F, t11798: F, t16720: F, t3284: F, t11387: F, t16676: F, t16677: F) -> (F, F, F, F, F, F) {
    let t33160 = t1084 * t33158 * t3415;
    let t33162 = t11784 * t9865;
    let t33165 = t28594 * t11379 * t11945;
    let t33167 = t11948 * t30095;
    let t33170 = t11798 * t3284 * t16720;
    let t33173 = t16676 * t11387 * t16677;
    (t33160, t33162, t33165, t33167, t33170, t33173)
}
