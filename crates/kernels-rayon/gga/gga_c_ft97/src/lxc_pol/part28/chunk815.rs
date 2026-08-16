//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 815/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk815(t144: f64, t32730: f64, t32732: f64, t1882: f64, t7397: f64, t32727: f64, t32993: f64, t2185: f64, t616: f64, t7312: f64, t558: f64, t574: f64, t7414: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t33009 = t144 * t32730;
    let t33012 = t144 * t32732;
    let t33016 = 2.0_f64 / 9.0_f64 * t1882 * t7397;
    let t33017 = t144 * t32727;
    let t33020 = t144 * t32993;
    let t33024 = t2185 * t616 * t7312;
    let t33028 = t574 * t7414 * t558;
    (t33009, t33012, t33016, t33017, t33020, t33024, t33028)
}
