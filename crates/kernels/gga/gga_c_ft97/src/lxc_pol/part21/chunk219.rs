//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 219/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk219<F: Float>(t1359: F, t586: F, t1369: F, t28: F, t526: F, t27: F, t89: F) -> (F, F, F, F, F) {
    let t1370 = t586 * t1359;
    let t1372 = t1369 * t28 * t1370;
    let t1374 = t526 * t1359;
    let t1376 = t89 * t27 * t1374;
    let t1378 = -t1372 / 6.0 - t1376 / 3.0;
    (t1370, t1372, t1374, t1376, t1378)
}
