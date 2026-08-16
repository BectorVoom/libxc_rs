//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 774/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk774(t10478: f64, t309: f64, t2349: f64, t824: f64, t4140: f64, t2347: f64, t870: f64, t875: f64, t4139: f64, t2680: f64, t665: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10479 = t10478 * t309;
    let t10480 = t2349 * t824;
    let t10481 = t4140 * t10480;
    let t10482 = t10479 * t10481;
    let t10485 = t870 * t2347;
    let t10486 = t2349 * t875;
    let t10487 = t10485 * t10486;
    let t10488 = t4139 * t10487;
    let t10491 = t665 * t2680;
    (t10479, t10480, t10481, t10482, t10485, t10486, t10487, t10488, t10491)
}
