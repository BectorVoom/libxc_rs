//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 705/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk705(t3: f64, t8113: f64, t1518: f64, t7553: f64, t117: f64, t7983: f64, t1916: f64, t1918: f64, t2113: f64, t2115: f64, t572: f64, t573: f64, param_d: f64) -> (f64, f64, f64, f64, f64) {
    let t8114 = t3 * t8113;
    let t8118 = param_d * t8113;
    let t8124 = t7553 * t1518;
    let t8127 = t117 * t7983;
    let t8130 = 3.0_f64 * t1916 * t2115 + 3.0_f64 * t1918 * t2113 + 6.0_f64 * t572 * t8124 + 3.0_f64 * t572 * t8127 + t573 * t8118;
    (t8114, t8118, t8124, t8127, t8130)
}
