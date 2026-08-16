//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 944/1331 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk944<F: Float>(t11442: F, t5553: F, t11326: F, t3714: F, t116: F, t190: F, t1: F, t102: F, t3694: F) -> (F, F, F, F) {
    let t11443 = t5553 * t11442;
    let t11445 = t11326 * t3714;
    let t11447 = t116 * t190;
    let t11449 = t3694 * t1 * t102;
    (t11443, t11445, t11447, t11449)
}
