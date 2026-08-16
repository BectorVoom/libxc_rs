//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 865/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk865(t1392: f64, t2958: f64, t1391: f64, t701: f64, t8469: f64, t1445: f64, t1835: f64, t8549: f64, t1865: f64, t1022: f64, t5750: f64, t3009: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8579 = t1392 * t2958;
    let t8580 = t1391 * t8579;
    let t8587 = t8469 * t701;
    let t8588 = t1445 * t8587;
    let t8591 = t2958 * t1835;
    let t8592 = t1445 * t8591;
    let t8595 = t1445 * t8549;
    let t8600 = t2958 * t1865;
    let t8601 = t1445 * t8600;
    let t8604 = t5750 * t1022;
    let t8605 = t8604 * t1865;
    let t8606 = t1445 * t8605;
    let t8612 = t3009 * t1865;
    (t8580, t8588, t8592, t8595, t8600, t8601, t8604, t8606, t8612)
}
