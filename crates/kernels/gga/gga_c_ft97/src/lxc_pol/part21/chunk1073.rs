//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1073/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1073<F: Float>(t100: F, t37429: F, t26177: F, t8392: F, t1882: F, t26326: F, t26185: F, t26464: F, t6559: F, t8232: F, t26387: F, t6544: F, t1851: F, t6557: F, t1326: F, t1587: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t102948 = t37429 * t100;
    let t102954 = 4.0 / 9.0 * t8392 * t26177;
    let t102960 = 4.0 / 9.0 * t1882 * t26326;
    let t102997 = 2.0 / 27.0 * t8392 * t26185;
    let t102999 = 2.0 / 9.0 * t1882 * t26464;
    let t103010 = t8232 * t6559;
    let t103013 = 2.0 / 9.0 * t1882 * t26387;
    let t103029 = t8232 * t6544;
    let t103068 = t1851 * t6557;
    let t103073 = t1587 * t1326;
    (t102948, t102954, t102960, t102997, t102999, t103010, t103013, t103029, t103068, t103073)
}
