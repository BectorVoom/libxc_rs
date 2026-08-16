//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 870/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk870(t281: f64, t6789: f64, t287: f64, t35384: f64, t213: f64, t6793: f64) -> (f64, f64, f64, f64) {
    let t35915 = t281 * t6789;
    let t35916 = t35384 * t287;
    let t35917 = t35915 * t35916;
    let t35924 = t213 * t6793;
    (t35915, t35916, t35917, t35924)
}
