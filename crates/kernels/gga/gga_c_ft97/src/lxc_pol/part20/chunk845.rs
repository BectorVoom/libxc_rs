//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 845/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk845<F: Float>(t299: F, t25443: F, t25498: F, t332: F, t5: F, t6399: F, t113: F, t1512: F, t1934: F, t2958: F, t2963: F, t2966: F, t505: F, t6400: F, t6403: F, t911: F, t1434: F, t681: F, t6891: F) -> (F, F, F, F, F) {
    let t300 = 10000000.0 <= t299;
    let t25499 = t25443 + t25498;
    let t25500 = t25499 * t332;
    let t25504 = t5 * t6399;
    let t25520 = piecewise3(t300, 0.0, t5 * t25500 * t113 / 4.0 + t25504 * t911 / 2.0 + t5 * t6400 * t505 / 2.0 + t6403 * t2958 / 4.0 + t6403 * t2963 / 4.0 + t6403 * t2966 / 2.0 + t5 * t1512 * t1934 / 4.0);
    let t27466 = t1434 * t681 * t6891;
    (t25499, t25500, t25504, t25520, t27466)
}
