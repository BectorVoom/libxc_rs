//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 1230/1331 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk1230(t11408: f64, t561: f64, t8951: f64, t11413: f64, t8960: f64, t19546: f64, t33623: f64, t5462: f64, t11224: f64, t518: f64, t13850: f64, t25042: f64) -> (f64, f64, f64, f64, f64) {
    let t35355 = t561 * t11408 * t8951;
    let t35358 = t561 * t11413 * t8960;
    let t35361 = t5462 * t33623 * t19546;
    let t35379 = t518 * t11224;
    let t35381 = t25042 * t13850;
    (t35355, t35358, t35361, t35379, t35381)
}
