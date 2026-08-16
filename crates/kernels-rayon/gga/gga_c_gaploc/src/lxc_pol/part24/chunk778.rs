//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 778/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk778(t1: f64, t7275: f64, t787: f64, t161: f64, t165: f64, t1835: f64, t969: f64, t2615: f64, t2617: f64, t826: f64, t2679: f64, t825: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7339 = t7275 * t1;
    let t7340 = t787 * t7339;
    let t7344 = t161 * t165 * t1835;
    let t7345 = t969 * t7344;
    let t7346 = t2615 * t7345;
    let t7348 = t826 * t2617;
    let t7349 = t2615 * t7348;
    let t7351 = t826 * t2679;
    let t7352 = t825 * t7351;
    (t7339, t7340, t7344, t7346, t7349, t7352)
}
