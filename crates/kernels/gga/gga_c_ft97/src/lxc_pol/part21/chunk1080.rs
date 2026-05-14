//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1080/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1080<F: Float>(t103: F, t25846: F, t101: F, t26373: F, t1882: F, t26262: F, t26142: F, t26230: F, t26398: F, t1326: F, t8275: F, t26368: F, t8392: F, t22892: F, t6414: F, t1286: F, t26129: F, t376: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t103864 = t103 * t25846;
    let t103872 = t101 * t26373;
    let t103881 = 4.0 / 9.0 * t1882 * t26262;
    let t103905 = 2.0 / 9.0 * t1882 * t26142;
    let t103918 = 4.0 / 9.0 * t1882 * t26230;
    let t103920 = 4.0 / 9.0 * t1882 * t26398;
    let t103927 = t8275 * t1326;
    let t103936 = 4.0 / 9.0 * t8392 * t26368;
    let t103955 = t6414 * t22892 / 9.0;
    let t103972 = 2.0 / 9.0 * t1286 * t376 * t26129;
    (t103864, t103872, t103881, t103905, t103918, t103920, t103927, t103936, t103955, t103972)
}
