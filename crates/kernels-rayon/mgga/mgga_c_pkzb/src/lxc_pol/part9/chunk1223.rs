//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1223/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1223(t21001: f64, t21004: f64, t21006: f64, t21008: f64, t21010: f64, t21012: f64, t21014: f64, t21016: f64, t21018: f64, t21021: f64, t21024: f64, t1954: f64, t723: f64, t730: f64, t7474: f64) -> (f64, f64) {
    let t21287 = -t21001 - t21004 - t21006 - t21008 - t21010 + t21012 + t21014 + t21016 + t21018 - t21021 - t21024;
    let t21291 = 0.35089341735807877242e1_f64 * t730 * t1954 * t7474 * t723;
    (t21287, t21291)
}
