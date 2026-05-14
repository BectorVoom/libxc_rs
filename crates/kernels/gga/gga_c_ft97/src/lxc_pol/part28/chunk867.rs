//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 867/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk867<F: Float>(t32917: F, t376: F, t5890: F, t1369: F, t32952: F, t1637: F, t7374: F, t7378: F, t1557: F, t7312: F, t72: F, t7369: F) -> (F, F, F, F, F, F, F, F) {
    let t139278 = t5890 * t376 * t32917;
    let t139312 = t1369 * t376 * t32952;
    let t139320 = t1369 * t1637 * t7374;
    let t139321 = 4.0 / 27.0 * t139320;
    let t139323 = t1369 * t1637 * t7378;
    let t139324 = 2.0 / 27.0 * t139323;
    let t139329 = t7312 * t1557;
    let t139352 = t72 * t7369;
    (t139278, t139312, t139320, t139321, t139323, t139324, t139329, t139352)
}
