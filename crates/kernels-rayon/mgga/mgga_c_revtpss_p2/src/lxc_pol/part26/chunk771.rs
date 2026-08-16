//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 771/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk771(t543: f64, t9890: f64, t1390: f64, t828: f64, t3926: f64, t3930: f64, t1398: f64, t3923: f64, t221: f64, t4019: f64, t4057: f64, t4018: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9891 = t9890 * t543;
    let t9893 = t1390 * t828 * t9891;
    let t9896 = t3930 * t3926;
    let t9898 = t3923 * t1398;
    let t9899 = t9898 * t543;
    let t9901 = t1390 * t828 * t9899;
    let t9905 = t4019 * t221 * t4057;
    let t9906 = t4018 * t9905;
    (t9891, t9893, t9896, t9898, t9899, t9901, t9905, t9906)
}
