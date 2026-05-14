//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 386/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk386<F: Float>(t325: F, t701: F, t723: F, t1901: F, t1077: F, t268: F, t61: F) -> (F, F, F, F) {
    let t1902 = t325 * t701;
    let t1903 = t1902 * t723;
    let t1904 = t1901 * t1903;
    let t1908 = t61 * t1077 * t268;
    (t1902, t1903, t1904, t1908)
}
