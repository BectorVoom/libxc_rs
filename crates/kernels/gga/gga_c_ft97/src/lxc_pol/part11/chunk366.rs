//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 366/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk366<F: Float>(t108: F, t1538: F, t1761: F, t1821: F, t1826: F, t1854: F, t1920: F, t1922: F, t438: F, t497: F, t88: F, t1580: F) -> (F, F) {
    let t1927 = -t108 * t1538 - t108 * t1761 - t1920 * t88 - 2.0 * t438 * t497 - 2.0 * t1821 - 4.0 * t1826 + 4.0 * t1854 + 2.0 * t1922;
    let t1934 = -t1580;
    (t1927, t1934)
}
