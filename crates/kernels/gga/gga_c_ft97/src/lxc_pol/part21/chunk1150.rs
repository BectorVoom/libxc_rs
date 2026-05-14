//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1150/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1150<F: Float>(t29701: F, t379: F, t22958: F, t5674: F, t101703: F, t3188: F, t25928: F, t23054: F, t29657: F, t1564: F, t23057: F, t4458: F, t1871: F, t22952: F, t25883: F, t25899: F) -> (F, F, F, F, F, F, F, F) {
    let t116338 = t29701 * t379;
    let t116340 = t5674 * t22958 * t116338;
    let t116342 = t101703 * t3188;
    let t116344 = t5674 * t25928 * t116342;
    let t116346 = t23054 * t29657;
    let t116347 = t116346 / 9.0;
    let t116350 = t5674 * t1564 * t23057 * t4458;
    let t116354 = t22952 * t1871 * t25899 * t25883;
    (t116338, t116340, t116342, t116344, t116346, t116347, t116350, t116354)
}
