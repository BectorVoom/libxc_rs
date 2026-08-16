//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 568/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk568(t1882: f64, t5970: f64, t5862: f64, t5871: f64, t5937: f64, t1384: f64, t358: f64, t1359: f64, t604: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t23484 = t1882 * t5970;
    let t23532 = t1882 * t5862;
    let t23534 = t1882 * t5871;
    let t23546 = t1882 * t5937;
    let t23548 = t1384 * t358;
    let t23571 = t604 * t1359;
    (t23484, t23532, t23534, t23546, t23548, t23571)
}
