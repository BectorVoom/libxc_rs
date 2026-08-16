//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 814/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk814(t103: f64, t7763: f64, t7800: f64, t1851: f64, t358: f64, t1570: f64, t2266: f64, t1557: f64, t8654: f64, t1736: f64, t179: f64, t171: f64, t7741: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11988 = t103 * t7763;
    let t12020 = t103 * t7800;
    let t12045 = t1851 * t358;
    let t12116 = t2266 * t1570;
    let t12122 = t8654 * t1557;
    let t12137 = t1736 * t179;
    let t12168 = 1.0_f64 / t171 / t7741;
    (t11988, t12020, t12045, t12116, t12122, t12137, t12168)
}
