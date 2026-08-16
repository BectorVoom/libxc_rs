//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1254/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1254(t15691: f64, t19985: f64, t1011: f64, t1068: f64, t15689: f64, t15700: f64, t19951: f64, t19954: f64, t19957: f64, t19960: f64, t19963: f64, t19968: f64, t19973: f64, t19977: f64, t19982: f64, t3106: f64, t4892: f64, t6331: f64) -> f64 {
    let t19986 = t15691 * t19985;
    let t19989 = t1011 * t19951 / 216.0_f64 + t1011 * t19954 / 108.0_f64 + 7.0_f64 / 648.0_f64 * t1011 * t19957 + t1011 * t19960 / 48.0_f64 - t1011 * t19963 / 72.0_f64 + 0.15244095330869239812e-2_f64 * t3106 * t6331 + 0.14291339372689912324e-3_f64 * t19968 * t1068 + 0.85748036236139473944e-3_f64 * t4892 * t19973 - 0.28582678745379824648e-3_f64 * t19977 + 0.47637797908966374413e-3_f64 * t15700 * t19982 - 0.28582678745379824648e-3_f64 * t15689 * t19986;
    t19989
}
