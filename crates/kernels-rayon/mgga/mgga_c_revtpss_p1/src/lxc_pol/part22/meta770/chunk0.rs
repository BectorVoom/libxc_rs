//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2855/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2855(t3431: f64, t408: f64, t3434: f64, t1126: f64, t12247: f64, t3800: f64, t3140: f64, t3552: f64, t3599: f64, t3362: f64, t3603: f64, t2251: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t44089 = t3431 * t3431;
    let t44091 = t408 / t44089;
    let t44092 = t3434 * t3434;
    let t44093 = 1.0_f64 / t44092;
    let t44101 = t1126 * t12247;
    let t44125 = t3800 * t3800;
    let t44126 = 1.0_f64 / t44125;
    let t44169 = t3552 * t3140;
    let t44170 = t44169 * t3599;
    let t44190 = t3603 * t3362;
    let t44191 = t44190 * t2251;
    (t44091, t44093, t44101, t44126, t44169, t44170, t44190, t44191)
}
