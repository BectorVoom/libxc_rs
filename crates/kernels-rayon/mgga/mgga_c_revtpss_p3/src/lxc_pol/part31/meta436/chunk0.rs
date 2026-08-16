//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1557/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1557(t15957: f64, t6266: f64, t3092: f64, t16509: f64, t4891: f64, t16584: f64, t1045: f64, t19497: f64, t3117: f64, t1043: f64, t11631: f64, t19450: f64) -> (f64, f64, f64, f64, f64) {
    let t19730 = t15957 * t6266;
    let t19731 = t3092 * t19730;
    let t19738 = t16509 * t4891;
    let t19741 = t16584 * t4891;
    let t19744 = t19497 * t1045;
    let t19745 = t3117 * t19744;
    let t19748 = t11631 * t1043;
    let t19749 = t19450 * t19748;
    (t19731, t19738, t19741, t19745, t19749)
}
