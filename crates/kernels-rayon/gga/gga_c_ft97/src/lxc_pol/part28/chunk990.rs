//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 990/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk990(t1369: f64, t32955: f64, t376: f64, t137087: f64, t7366: f64, t7370: f64, t1389: f64, t5842: f64, t2178: f64, t7390: f64, t24073: f64, t7309: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t139526 = t1369 * t376 * t32955;
    let t139533 = t7366 * t137087 * t7370;
    let t139534 = 10.0_f64 / 27.0_f64 * t139533;
    let t139563 = t5842 * t1389;
    let t139573 = t7390 * t2178;
    let t139600 = t7309 * t24073;
    (t139526, t139533, t139534, t139563, t139573, t139600)
}
