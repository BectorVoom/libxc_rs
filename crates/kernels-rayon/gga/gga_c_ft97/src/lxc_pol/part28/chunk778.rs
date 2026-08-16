//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 778/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk778(t1871: f64, t32562: f64, t488: f64, t110: f64, t32082: f64, t1332: f64, t5617: f64, t452: f64, t379: f64, t447: f64, t7288: f64, t103: f64, t32365: f64, t82: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t32564 = t1871 * t488 * t32562;
    let t32568 = t1871 * t110 * t32082;
    let t32571 = t5617 * t1332;
    let t32573 = t452 * t488 * t32571;
    let t32577 = t447 * t7288 * t379;
    let t32581 = t82 * t32365 * t103;
    (t32564, t32568, t32571, t32573, t32577, t32581)
}
