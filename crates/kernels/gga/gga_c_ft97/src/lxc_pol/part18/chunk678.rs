//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 678/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk678<F: Float>(t11604: F, t3187: F, t1909: F, t1882: F, t3277: F, t3273: F, t1853: F, t942: F, t1852: F, t452: F, t1588: F) -> (F, F, F, F, F, F, F) {
    let t11605 = t3187 * t11604;
    let t11606 = t1909 * t11605;
    let t11610 = 2.0 / 27.0 * t1882 * t3277;
    let t11612 = 2.0 / 9.0 * t1882 * t3273;
    let t11613 = t942 * t1853;
    let t11615 = t452 * t1852 * t11613;
    let t11618 = t942 * t1588;
    (t11605, t11606, t11610, t11612, t11613, t11615, t11618)
}
