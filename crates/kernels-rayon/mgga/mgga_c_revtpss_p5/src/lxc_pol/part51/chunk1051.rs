//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1051/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1051(t31912: f64, t31965: f64, t31908: f64, t31949: f64, t1032: f64, t25698: f64, t31919: f64, t25638: f64, t8513: f64, t120304: f64, t1982: f64, t3316: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t120403 = t31912 * t31965;
    let t120406 = t31908 * t31949;
    let t120412 = t25698 * t1032;
    let t120419 = t31919 * t31965;
    let t120425 = t8513 * t25638;
    let t120429 = t1982 * t3316 * t120304;
    (t120403, t120406, t120412, t120419, t120425, t120429)
}
