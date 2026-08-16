//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 960/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk960(t1668: f64, t3154: f64, t19572: f64, t3117: f64, t357: f64, t15696: f64, t6267: f64, t23503: f64, t4915: f64, t11890: f64, t15189: f64, t18919: f64, t18924: f64, t18934: f64, t23479: f64, t23483: f64, t23487: f64, t23490: f64, t23501: f64, t23505: f64) -> (f64, f64, f64, f64, f64) {
    let t23929 = t3154 * t1668;
    let t23930 = t19572 * t23929;
    let t23931 = t3117 * t23930;
    let t23934 = t1668 * t357;
    let t23935 = t19572 * t23934;
    let t23936 = t3117 * t23935;
    let t23939 = t15696 * t6267;
    let t23945 = t4915 * t23503;
    let t23958 = -t11890 - 0.11111111111111111111e-1_f64 * t15189 + 0.55555555555555555555e-2_f64 * t18919 - 0.16666666666666666667e-1_f64 * t18924 + 0.83333333333333333334e-2_f64 * t18934 - 0.92592592592592592592e-2_f64 * t23479 + 0.33333333333333333333e-1_f64 * t23483 - 0.16666666666666666666e-1_f64 * t23501 - 0.50000000000000000001e-1_f64 * t23487 + 0.50000000000000000001e-1_f64 * t23505 - 0.83333333333333333333e-2_f64 * t23490;
    (t23931, t23936, t23939, t23945, t23958)
}
