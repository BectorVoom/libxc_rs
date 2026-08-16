//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 327/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk327(t332: f64, t4375: f64, t113: f64, t1273: f64, t909: f64, t1274: f64, t505: f64, t910: f64, t992: f64, t18: f64, t1577: f64, t2321: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4376 = t4375 * t332;
    let t4377 = t4376 * t113;
    let t4380 = t1273 * t909;
    let t4381 = t332 * t113;
    let t4382 = t4380 * t4381;
    let t4385 = t1274 * t505;
    let t4391 = t910 * t992;
    let t4394 = t332 * t18;
    let t4395 = t4394 * t1577;
    let t4906 = t2321 * t992;
    (t4377, t4382, t4385, t4391, t4395, t4906)
}
