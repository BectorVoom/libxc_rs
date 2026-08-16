//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 980/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk980(t30543: f64, t8473: f64, t31419: f64, t4810: f64, t721: f64, t1503: f64, t7329: f64, t1992: f64, t5616: f64, t7585: f64, t7586: f64, t10146: f64, t167: f64, t576: f64) -> (f64, f64, f64, f64, f64) {
    let t34640 = t30543 * t8473;
    let t34650 = t31419 * t4810 * t721;
    let t34659 = t7329 * t1503;
    let t34660 = 7.0_f64 / 72.0_f64 * t34659;
    let t34675 = t7585 * t7586 * t1992 * t5616;
    let t34691 = t576 * t167 * t10146;
    (t34640, t34650, t34660, t34675, t34691)
}
