//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1062/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1062(t41831: f64, t41835: f64, t41839: f64, t41844: f64, t41846: f64, t41852: f64, t41855: f64, t41859: f64, t41863: f64, t41867: f64, t41870: f64, t41873: f64, t41877: f64, t41882: f64, t41886: f64) -> f64 {
    let t42025 = 8.0_f64 / 9.0_f64 * t41831 - 4.0_f64 / 3.0_f64 * t41835 - 4.0_f64 / 3.0_f64 * t41839 + 2.0_f64 * t41844 + 8.0_f64 / 3.0_f64 * t41846 + 8.0_f64 * t41852 + 16.0_f64 / 9.0_f64 * t41855 + 2.0_f64 / 3.0_f64 * t41859 + 4.0_f64 / 9.0_f64 * t41863 + 8.0_f64 / 3.0_f64 * t41867 - 4.0_f64 * t41870 + 8.0_f64 / 3.0_f64 * t41873 - 4.0_f64 / 9.0_f64 * t41877 - 8.0_f64 / 9.0_f64 * t41882 - 16.0_f64 / 9.0_f64 * t41886;
    t42025
}
