//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 1091/1327 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk1091<F: Float>(t1084: F, t15516: F, t33415: F, t3708: F, t9563: F, t9934: F, t11387: F, t15650: F, t7204: F, t8785: F, t8910: F, t15610: F, t2660: F) -> (F, F, F, F, F) {
    let t33417 = t1084 * t33415 * t15516;
    let t33420 = t9563 * t3708 * t9934;
    let t33427 = t7204 * t11387 * t15650;
    let t33429 = t8910 * t8785;
    let t33431 = t2660 * t33429 * t15610;
    (t33417, t33420, t33427, t33429, t33431)
}
