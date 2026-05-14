//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 775/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk775<F: Float>(t1317: F, t1318: F, t1637: F, t376: F, t5696: F, t89: F, t1316: F, t458: F) -> (F, F, F, F) {
    let t23037 = t1317 * t1637 * t1318;
    let t23038 = 2.0 / 9.0 * t23037;
    let t23047 = t376 * t5696;
    let t23048 = t89 * t23047;
    let t23054 = t1316 * t458;
    (t23037, t23038, t23048, t23054)
}
