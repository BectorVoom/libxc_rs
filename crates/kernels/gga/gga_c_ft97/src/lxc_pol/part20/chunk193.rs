//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 193/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk193<F: Float>(t1268: F, t898: F, t900: F, t1263: F, t631: F, t892: F, t332: F, t113: F, t2: F, t661: F, t4: F) -> (F, F, F, F, F, F) {
    let t1270 = t898 * t900 * t1268;
    let t1273 = t892 + t631 * t1263 / 6.0 + t631 * t1270 / 2.0;
    let t1274 = t1273 * t332;
    let t1275 = t1274 * t113;
    let t1401 = t661 * t2;
    let t1402 = t1401 * t4;
    (t1270, t1273, t1274, t1275, t1401, t1402)
}
