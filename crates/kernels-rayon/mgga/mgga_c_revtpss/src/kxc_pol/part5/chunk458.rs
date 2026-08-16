//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 458/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk458(t1610: f64, t915: f64, t1594: f64, t939: f64, t1601: f64, t1604: f64, t1607: f64, t948: f64, t951: f64, t954: f64) -> (f64, f64, f64, f64) {
    let t1612 = 1.0_f64 * t915 * t1610;
    let t1614 = -t939 - 0.17123333333333333333e-1_f64 * t1594;
    let t1621 = 0.3529725e1_f64 * t1601 - t948 - 0.516475e0_f64 * t1594 + 0.6311625e0_f64 * t1604 - t951 - 0.104195e0_f64 * t1607;
    let t1622 = t1621 * t954;
    (t1612, t1614, t1621, t1622)
}
