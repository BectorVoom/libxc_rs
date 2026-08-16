//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 540/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk540<F: Float>(t1573: F, t942: F, t1580: F, t2932: F, t300: F, t2904: F, t1592: F, t2970: F, t973: F, t2978: F, t60: F, t344: F) -> (F, F, F, F, F, F, F) {
    let t4449 = t1573 * t942;
    let t4475 = t1580 * t2932;
    let t4483 = t300 * t1573;
    let t4488 = t2904 * t1580;
    let t4506 = t2970 * t1592;
    let t4507 = t973 * t4506;
    let t4509 = t60 * t2978;
    let t4510 = t4509 * t344;
    (t4449, t4475, t4483, t4488, t4507, t4509, t4510)
}
