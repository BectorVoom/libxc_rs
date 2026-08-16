//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 952/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk952(t11560: f64, t15189: f64, t18919: f64, t18924: f64, t18934: f64, t23479: f64, t23483: f64, t23487: f64, t23490: f64, t23501: f64, t23505: f64, t324: f64) -> f64 {
    let t23811 = -t11560 - 0.12361111111111111111e-1_f64 * t15189 + 0.61805555555555555556e-2_f64 * t18919 - 0.18541666666666666667e-1_f64 * t18924 + 0.92708333333333333334e-2_f64 * t18934 - 0.10300925925925925926e-1_f64 * t23479 + 0.37083333333333333333e-1_f64 * t23483 - 0.18541666666666666666e-1_f64 * t23501 - 0.55625000000000000001e-1_f64 * t23487 + 0.55625000000000000001e-1_f64 * t23505 - 0.92708333333333333333e-2_f64 * t23490;
    let t23812 = t23811 * t324;
    t23812
}
