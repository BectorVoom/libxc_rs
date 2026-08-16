//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 611/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk611(t27869: f64, t446: f64, t6109: f64, t681: f64, t6879: f64, t1434: f64, t6887: f64, t6837: f64, t713: f64) -> (f64, f64, f64, f64) {
    let t27870 = t446 * t27869;
    let t27873 = t6109 * t681 * t6879;
    let t27876 = t1434 * t681 * t6887;
    let t27878 = t6837 * t713;
    (t27870, t27873, t27876, t27878)
}
