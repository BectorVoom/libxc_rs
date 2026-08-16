//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 1204/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk1204(t144: f64, t21281: f64, t21283: f64, t33267: f64, t11537: f64, t20372: f64, t5059: f64, t1: f64, t1457: f64, t169: f64, t1736: f64, t11344: f64) -> (f64, f64, f64, f64) {
    let t34918 = t21281 * t33267 * t144 * t21283;
    let t34921 = t11537 * t20372 * t5059;
    let t34925 = t169 * t1457 * t1736 * t1;
    let t34926 = t34925 * t11344;
    (t34918, t34921, t34925, t34926)
}
