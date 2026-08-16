//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 363/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk363(t363: f64, t432: f64, t1903: f64, t1902: f64, t100: f64, t463: f64) -> (f64, f64, f64, f64) {
    let t1904 = t363 * t432;
    let t1905 = t1903 * t1904;
    let t1906 = t1902 * t1905;
    let t1909 = t463 * t100;
    (t1904, t1905, t1906, t1909)
}
