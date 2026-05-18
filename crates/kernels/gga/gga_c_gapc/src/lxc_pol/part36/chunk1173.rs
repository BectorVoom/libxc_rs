//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 1173/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk1173<F: Float>(t11488: F, t1771: F, t11495: F, t1723: F, t11500: F, t11356: F, t3060: F, t9262: F, t11303: F, t19530: F, t11302: F, t5285: F) -> (F, F, F, F, F, F) {
    let t34595 = t11488 * t1771;
    let t34597 = t11495 * t1723;
    let t34599 = t11500 * t1723;
    let t34602 = t3060 * t11356 * t9262;
    let t34605 = t11303 * t19530;
    let t34607 = t5285 * t11302;
    (t34595, t34597, t34599, t34602, t34605, t34607)
}
