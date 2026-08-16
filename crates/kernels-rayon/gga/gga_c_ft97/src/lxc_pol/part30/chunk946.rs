//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 946/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk946(t33452: f64, t668: f64, t1434: f64, t2399: f64, t7520: f64, t2248: f64, t322: f64, t7511: f64, t7516: f64, t2253: f64, t6108: f64, t33296: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t141357 = t33452 * t668;
    let t141363 = t1434 * t2399 * t7520;
    let t141364 = 4.0_f64 / 9.0_f64 * t141363;
    let t141365 = t2248 * t322;
    let t141367 = t7511 * t141365 * t7516;
    let t141368 = 10.0_f64 / 9.0_f64 * t141367;
    let t141369 = t6108 * t2253;
    let t141370 = t141369 * t33296;
    (t141357, t141363, t141364, t141365, t141367, t141368, t141369, t141370)
}
