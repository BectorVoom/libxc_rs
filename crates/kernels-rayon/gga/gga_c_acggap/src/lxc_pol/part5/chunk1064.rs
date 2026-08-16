//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1064/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1064(t330: f64, t5291: f64, t3207: f64, t509: f64, t3382: f64, t4316: f64, t1016: f64, t1410: f64, t1451: f64, t3228: f64, t1005: f64, t4503: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18828 = t330 * t5291;
    let t18830 = t3207 * t509;
    let t18832 = t3382 * t4316;
    let t18834 = t1016 * t1410;
    let t18839 = t3228 * t1451;
    let t18841 = t1005 * t4503;
    (t18828, t18830, t18832, t18834, t18839, t18841)
}
