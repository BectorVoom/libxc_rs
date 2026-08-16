//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1319/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1319(t1662: f64, t105: f64, t11828: f64, t11834: f64, t11837: f64, t19999: f64, t20000: f64, t20001: f64, t20002: f64, t20003: f64, t20004: f64, t20005: f64, t6583: f64, t694: f64, t814: f64, t839: f64, t96: f64) -> f64 {
    let t24582 = t1662 * t1662;
    let t24587 = -2.0_f64 * t105 * t24582 * t814 * t96 - 3.0_f64 * t6583 * t694 * t839 + t11828 - t11834 + t11837 - t19999 - t20000 - t20001 - t20002 - t20003 - t20004 - t20005;
    t24587
}
