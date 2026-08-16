//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1105/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1105(t1518: f64, t2051: f64, t2055: f64, t4147: f64, t8107: f64, t211: f64, t9644: f64, t138: f64, t785: f64, t9302: f64, t2452: f64, t9720: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34251 = t2051 * t1518;
    let t34359 = t1518 * t2055;
    let t34495 = t4147 * t8107;
    let t39643 = 1.0_f64 / t9644 / t211;
    let t40270 = t138 * t9302 * t785;
    let t40688 = t9720 * t2452;
    (t34251, t34359, t34495, t39643, t40270, t40688)
}
