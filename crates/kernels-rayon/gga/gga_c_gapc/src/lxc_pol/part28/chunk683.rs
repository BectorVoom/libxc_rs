//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 683/1429 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk683(t103: f64, t1906: f64, t203: f64, t5698: f64, t5700: f64, t169: f64, t4048: f64, t4054: f64, t442: f64, t505: f64, t682: f64, t3141: f64) -> (f64, f64, f64, f64) {
    let t5703 = t1906 * t5698 * t203 * t5700 * t103;
    let t5708 = t169 * t4048;
    let t5713 = t4054 * t442;
    let t5721 = t682 * t505;
    let t5722 = t3141 * t5721;
    (t5703, t5708, t5713, t5722)
}
