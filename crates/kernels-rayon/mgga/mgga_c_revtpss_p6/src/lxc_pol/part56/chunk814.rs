//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 814/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk814(t2134: f64, t3682: f64, t1234: f64, t7623: f64, t1210: f64, t8945: f64, t487: f64, t7642: f64) -> (f64, f64, f64, f64) {
    let t26877 = t2134 * t3682 / 432.0_f64;
    let t26880 = t1234 * t7623;
    let t26889 = t1210 * t8945;
    let t26894 = t7642 * t487;
    (t26877, t26880, t26889, t26894)
}
