//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 802/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk802(t3057: f64, t7143: f64, t1035: f64, t8515: f64, t1983: f64, t378: f64, t7150: f64) -> (f64, f64, f64) {
    let t25591 = t3057 * t7143;
    let t25604 = t8515 * t1035;
    let t25605 = t1983 * t25604;
    let t25610 = t7150 * t378;
    (t25591, t25605, t25610)
}
