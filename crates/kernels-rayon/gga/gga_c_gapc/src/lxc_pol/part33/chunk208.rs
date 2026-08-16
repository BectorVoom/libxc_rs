//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 208/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk208(t282: f64, t768: f64, t122: f64, t766: f64, t291: f64, t435: f64) -> (f64, f64, f64, f64) {
    let t769 = t768 * t282;
    let t770 = t769 * t122;
    let t771 = t766 * t770;
    let t772 = t435 * t291;
    (t769, t770, t771, t772)
}
