//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 717/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk717(t11801: f64, t13609: f64, t3009: f64, t3431: f64, t1445: f64, t11832: f64, t935: f64, t5748: f64, t11894: f64, t2087: f64, t123: f64, t3601: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13611 = 0.42900587942220512003e1_f64 * t11801 * t13609;
    let t13612 = t3009 * t3431;
    let t13613 = t1445 * t13612;
    let t13616 = t11832 * t935;
    let t13617 = t1445 * t13616;
    let t13619 = 0.27606906686822939767e2_f64 * t5748 * t13617;
    let t13620 = t11894 * t935;
    let t13621 = t1445 * t13620;
    let t13623 = 0.69017266717057349418e1_f64 * t2087 * t13621;
    let t13624 = t3601 * t123;
    (t13611, t13612, t13613, t13616, t13617, t13619, t13620, t13621, t13623, t13624)
}
