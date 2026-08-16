//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 738/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk738(t4757: f64, t996: f64, t1096: f64, t1651: f64, t1079: f64, t2848: f64, t3070: f64, t4571: f64, t4576: f64, t4581: f64, t4585: f64) -> (f64, f64, f64) {
    let t4758 = t996 * t4757;
    let t4763 = t1651 * t1096;
    let t4764 = t1079 * t4763;
    let t4772 = t3070 + 0.4938888888888888889e-2_f64 * t2848 + 0.4938888888888888889e-2_f64 * t4571 - 0.9877777777777777778e-2_f64 * t4576 + 0.29633333333333333334e-1_f64 * t4581 - 0.14816666666666666667e-1_f64 * t4585;
    (t4758, t4764, t4772)
}
