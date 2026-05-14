//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1108/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1108<F: Float>(t1882: F, t27295: F, t26880: F, t26885: F, t6710: F, t8232: F, t26846: F, t27012: F, t8392: F, t27232: F, t26936: F, t27239: F, t26973: F, t26851: F, t27246: F, t158: F, t23455: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t107156 = 2.0 / 9.0 * t1882 * t27295;
    let t107168 = 2.0 / 9.0 * t1882 * t26880;
    let t107170 = 2.0 / 9.0 * t1882 * t26885;
    let t107177 = t8232 * t6710;
    let t107180 = 2.0 / 9.0 * t1882 * t26846;
    let t107183 = 4.0 / 9.0 * t8392 * t27012;
    let t107193 = 4.0 / 27.0 * t8392 * t27232;
    let t107210 = 2.0 / 27.0 * t8392 * t26936;
    let t107234 = 4.0 / 27.0 * t8392 * t27239;
    let t107236 = 2.0 / 9.0 * t1882 * t26973;
    let t107241 = 2.0 / 9.0 * t1882 * t26851;
    let t107243 = 2.0 / 27.0 * t8392 * t27246;
    let t107273 = t158 * t23455;
    (t107156, t107168, t107170, t107177, t107180, t107183, t107193, t107210, t107234, t107236, t107241, t107243, t107273)
}
