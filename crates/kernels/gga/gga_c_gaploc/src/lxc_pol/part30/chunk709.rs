//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 709/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk709<F: Float>(t2367: F, t549: F, t1429: F, t1265: F, t2366: F, t2365: F, t1359: F, t874: F) -> (F, F, F) {
    let t6596 = t549 * t2367;
    let t6597 = t1429 * t6596;
    let t6599 = t2366 * t1265;
    let t6600 = t2365 * t6599;
    let t6601 = t1429 * t6600;
    let t6603 = t1359 * t874;
    (t6597, t6601, t6603)
}
