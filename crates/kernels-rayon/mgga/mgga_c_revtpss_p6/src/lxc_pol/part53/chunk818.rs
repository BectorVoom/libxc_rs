//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 818/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk818(t1936: f64, t7586: f64, t8559: f64, t8562: f64, t8564: f64, t8741: f64, t196: f64, t2165: f64, t197: f64) -> (f64, f64, f64) {
    let t8758 = t7586 * t1936;
    let t8761 = t8741 + 2.0_f64 * t8758 + 2.0_f64 * t8559 + t8562 + t8564;
    let t8763 = t2165 * t196;
    let t8764 = t8763 * t197;
    (t8761, t8763, t8764)
}
