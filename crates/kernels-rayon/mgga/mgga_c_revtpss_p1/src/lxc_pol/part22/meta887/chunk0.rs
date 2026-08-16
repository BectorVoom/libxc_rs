//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3074/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3074(t1633: f64, t3012: f64, t2986: f64, t4682: f64, t11465: f64, t1626: f64, t15234: f64, t3014: f64, t11509: f64, t4707: f64, t11385: f64, t1609: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t52430 = t3012 * t1633;
    let t52440 = t4682 * t2986;
    let t52443 = t1626 * t11465;
    let t52452 = t15234 * t3014;
    let t52459 = t4707 * t11509;
    let t52482 = t11385 * t1609;
    (t52430, t52440, t52443, t52452, t52459, t52482)
}
