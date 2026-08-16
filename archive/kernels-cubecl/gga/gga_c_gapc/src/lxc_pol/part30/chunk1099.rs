//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 1099/1331 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk1099<F: Float>(t1084: F, t33273: F, t9415: F, t188: F, t20: F, t5658: F, t10293: F, t29664: F, t3437: F, t11449: F, t11815: F, t190: F, t2786: F) -> (F, F, F, F, F) {
    let t33494 = t1084 * t33273;
    let t33495 = t33494 * t9415;
    let t33498 = t20 * t5658 * t188;
    let t33501 = t3437 * t33498 * t10293 * t29664;
    let t33505 = t2786 * t190 * t11449 * t11815;
    (t33494, t33495, t33498, t33501, t33505)
}
