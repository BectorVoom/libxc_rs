//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 355/1331 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk355<F: Float>(t1599: F, t514: F, t19: F, t203: F, t147: F, t567: F) -> (F, F, F, F) {
    let t1600 = t514 * t1599;
    let t1601 = t203 * t19;
    let t1602 = t147 * t567;
    let t1603 = t1601 * t1602;
    (t1600, t1601, t1602, t1603)
}
