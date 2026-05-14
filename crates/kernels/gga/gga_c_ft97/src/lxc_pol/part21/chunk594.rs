//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 594/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk594<F: Float>(t487: F, t942: F, t1882: F, t3231: F, t3201: F, t8392: F, t3170: F) -> (F, F, F, F) {
    let t11811 = t487 * t942;
    let t11821 = 2.0 / 9.0 * t1882 * t3231;
    let t11826 = 2.0 / 27.0 * t8392 * t3201;
    let t11837 = t3170 * t487;
    (t11811, t11821, t11826, t11837)
}
