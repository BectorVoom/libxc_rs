//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1066/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1066(t1040: f64, t19696: f64, t16509: f64, t4891: f64, t16584: f64, t19463: f64, t366: f64, t11710: f64, t6267: f64, t3091: f64, t3172: f64, t6311: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t19697 = t19696 * t1040;
    let t19738 = t16509 * t4891;
    let t19741 = t16584 * t4891;
    let t19773 = t19463 * t366;
    let t19785 = t11710 * t6267;
    let t19786 = t3091 * t19785;
    let t19826 = t3172 * t6311;
    (t19697, t19738, t19741, t19773, t19785, t19786, t19826)
}
