//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1022/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1022(t1086: f64, t3057: f64, t3090: f64, t11671: f64, t3114: f64, t11200: f64, t225: f64, t1053: f64, t3204: f64, t1021: f64, t3201: f64, t1054: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11926 = t3057 * t1086;
    let t11927 = t11926 * t3090;
    let t11933 = t3114 * t11671;
    let t11940 = t11200 * t225;
    let t11947 = t3204 * t1053;
    let t11956 = t1021 * t3201;
    let t11967 = t1054 * t3201;
    (t11927, t11933, t11940, t11947, t11956, t11967)
}
