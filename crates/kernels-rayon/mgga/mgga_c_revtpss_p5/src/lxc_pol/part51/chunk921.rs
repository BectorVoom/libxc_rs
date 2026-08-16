//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 921/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk921(t11922: f64, t8523: f64, t8522: f64, t1078: f64, t1096: f64, t247: f64, t3116: f64, t7165: f64, t8513: f64) -> (f64, f64, f64, f64, f64) {
    let t31883 = t8523 * t11922;
    let t31885 = 0.12395776403017003607e-3_f64 * t8522 * t31883;
    let t31886 = t1078 * t1096;
    let t31888 = t247 * t3116 * t31886;
    let t31891 = t8513 * t7165;
    (t31883, t31885, t31886, t31888, t31891)
}
