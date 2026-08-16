//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 787/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk787(t1918: f64, t2170: f64, t573: f64, t7949: f64, t7952: f64, t7955: f64, t8245: f64, t38: f64, t73: f64, t74: f64, t84: f64) -> (f64, f64, f64, f64, f64) {
    let t8249 = 3.0_f64 * t1918 * t2170 + t573 * t8245 + t7949 + t7952 + t7955;
    let t8435 = t38 * t38;
    let t8440 = 1.0_f64 / t74 / t73;
    let t8441 = t84 * t84;
    let t8442 = t8440 * t8441;
    (t8249, t8435, t8440, t8441, t8442)
}
