//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 589/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk589<F: Float>(t1882: F, t3263: F, t3240: F, t3235: F, t8232: F, t981: F, t110: F, t8326: F) -> (F, F, F, F, F) {
    let t11535 = 2.0 / 9.0 * t1882 * t3263;
    let t11537 = 2.0 / 9.0 * t1882 * t3240;
    let t11549 = 2.0 / 9.0 * t1882 * t3235;
    let t11550 = t8232 * t981;
    let t11552 = t8326 * t110;
    (t11535, t11537, t11549, t11550, t11552)
}
