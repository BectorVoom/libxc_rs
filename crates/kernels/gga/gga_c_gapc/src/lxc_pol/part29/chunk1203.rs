//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 1203/1311 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk1203<F: Float>(t1765: F, t3670: F, t11391: F, t3163: F, t128: F, t203: F, t11417: F, t457: F, t5741: F, t632: F, t1781: F, t3684: F) -> (F, F, F, F, F) {
    let t34858 = t3670 * t1765;
    let t34860 = t11391 * t3163;
    let t34863 = t203 * t128;
    let t34866 = t632 * t11417 * t5741 * t34863 * t457;
    let t34868 = t3684 * t1781;
    (t34858, t34860, t34863, t34866, t34868)
}
