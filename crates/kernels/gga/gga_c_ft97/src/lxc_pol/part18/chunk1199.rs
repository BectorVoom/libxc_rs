//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1199/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1199<F: Float>(t1557: F, t6454: F, t1559: F, t446: F, t7793: F, t1882: F, t25957: F, t6513: F, t8232: F, t12001: F, t25961: F, t101573: F, t27: F, t370: F, t89: F, t25943: F) -> (F, F, F, F, F, F, F) {
    let t101703 = t6454 * t1557;
    let t101706 = t446 * t7793 * t101703 * t1559;
    let t101708 = t1882 * t25957;
    let t101709 = 2.0 / 27.0 * t101708;
    let t101710 = t8232 * t6513;
    let t101712 = t12001 * t25961;
    let t101716 = t89 * t27 * t370 * t101573;
    let t101718 = t1882 * t25943;
    (t101706, t101708, t101709, t101710, t101712, t101716, t101718)
}
