//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 896/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk896(t6244: f64, t996: f64, t1651: f64, t1695: f64, t1079: f64, t3070: f64, t4571: f64, t6094: f64, t6098: f64, t6102: f64) -> (f64, f64, f64) {
    let t6245 = t996 * t6244;
    let t6250 = t1651 * t1695;
    let t6251 = t1079 * t6250;
    let t6258 = t3070 + 0.9877777777777777778e-2_f64 * t4571 - 0.9877777777777777778e-2_f64 * t6094 + 0.29633333333333333334e-1_f64 * t6098 - 0.14816666666666666667e-1_f64 * t6102;
    (t6245, t6251, t6258)
}
