//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 721/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk721(t1918: f64, t2170: f64, t573: f64, t7949: f64, t7952: f64, t7955: f64, t8245: f64, t38: f64) -> (f64, f64) {
    let t8249 = 3.0_f64 * t1918 * t2170 + t573 * t8245 + t7949 + t7952 + t7955;
    let t8435 = t38 * t38;
    (t8249, t8435)
}
