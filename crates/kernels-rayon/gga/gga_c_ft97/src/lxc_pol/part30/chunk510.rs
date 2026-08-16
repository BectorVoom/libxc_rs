//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 510/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk510(t1609: f64, t2378: f64, t2427: f64, t6: f64, t224: f64, t1095: f64, t2393: f64, t51: f64, t6032: f64, t3771: f64, t200: f64, t709: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13411 = t1609 * t2378;
    let t13442 = t2427 * t6;
    let t13443 = t224 * t13442;
    let t13469 = t2378 * t1095;
    let t13475 = t2393 * t1095;
    let t13519 = t6032 * t51;
    let t13520 = t3771 * t13519;
    let t13521 = t200 * t709;
    (t13411, t13442, t13443, t13469, t13475, t13519, t13520, t13521)
}
