//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1277/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1277(t27833: f64, t8715: f64, t32735: f64, t7898: f64, t125362: f64, t125365: f64, t128945: f64, t128959: f64, t128960: f64, t128964: f64, t1932: f64, t2056: f64, t28030: f64, t28586: f64, t32322: f64, t33602: f64, t651: f64, t6983: f64, t7367: f64, t7373: f64, t7374: f64, t7883: f64, t8065: f64, t8109: f64) -> f64 {
    let t128965 = t27833 * t8715;
    let t128966 = t7898 * t32735;
    let t128967 = -2.0_f64 * t651 * t7373 * t7883 - 2.0_f64 * t125362 * t2056 - 2.0_f64 * t125365 * t2056 - t1932 * t28586 - 2.0_f64 * t28030 * t7374 + t32322 * t8109 - 2.0_f64 * t33602 * t7367 - t6983 * t8065 + t128945 + t128959 + t128960 + t128964 + t128965 + t128966;
    t128967
}
