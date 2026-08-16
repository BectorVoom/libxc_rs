//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1602/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1602(t1150: f64, t12248: f64, t44018: f64, t3384: f64, t44097: f64, t1188: f64, t1196: f64, t3495: f64, t43966: f64, t3798: f64, t3800: f64, t3140: f64, t3552: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t44111 = 24.0_f64 * t12248 * t44018 * t1150;
    let t44114 = 6.0_f64 * t3384 * t44097 * t1150;
    let t44122 = 0.35089341735807877242e1_f64 * t1196 * t3495 * t43966 * t1188;
    let t44123 = t3798 * t3798;
    let t44125 = t3800 * t3800;
    let t44126 = 1.0_f64 / t44125;
    let t44169 = t3552 * t3140;
    (t44111, t44114, t44122, t44123, t44126, t44169)
}
