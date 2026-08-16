//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1219/1427 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1219(t1: f64, t26662: f64, t5462: f64, t8681: f64, t11332: f64, t1643: f64, t4995: f64, t11347: f64, t620: f64, t1929: f64, t3670: f64, t11537: f64, t3137: f64, t505: f64, t5059: f64, t674: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34419 = t26662 * t1;
    let t34421 = t5462 * t34419 * t8681;
    let t34424 = t1643 * t4995 * t11332;
    let t34426 = t11347 * t620;
    let t34428 = t3670 * t1929;
    let t34433 = t11537 * t3137 * t505 * t674 * t5059;
    (t34419, t34421, t34424, t34426, t34428, t34433)
}
