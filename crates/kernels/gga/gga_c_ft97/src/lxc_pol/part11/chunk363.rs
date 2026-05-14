//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 363/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk363<F: Float>(t103: F, t358: F, t363: F, t432: F, t1902: F, t100: F, t463: F) -> (F, F, F, F, F) {
    let t1903 = t103 * t358;
    let t1904 = t363 * t432;
    let t1905 = t1903 * t1904;
    let t1906 = t1902 * t1905;
    let t1909 = t463 * t100;
    (t1903, t1904, t1905, t1906, t1909)
}
