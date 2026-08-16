//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 916/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk916(t38776: f64, t38792: f64, t38809: f64, t38825: f64, t1882: f64, t8383: f64, t8388: f64, t8392: f64, t482: f64, t7943: f64, t89: f64, t480: f64, t8326: f64) -> (f64, f64, f64, f64, f64) {
    let t38827 = t38776 + t38792 + t38809 + t38825;
    let t38833 = t1882 * t8383;
    let t38842 = t8392 * t8388;
    let t38846 = t89 * t7943 * t482;
    let t38866 = t8326 * t480;
    (t38827, t38833, t38842, t38846, t38866)
}
