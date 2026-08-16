//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2488/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2488(t43813: f64, t43816: f64, t3431: f64, t408: f64, t3434: f64, t1126: f64, t12247: f64, t3800: f64, t12773: f64, t12784: f64, t12772: f64, t12835: f64, t3625: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t44039 = 0.31003950617283950618e1_f64 * t43813;
    let t44040 = 0.13388493827160493828e1_f64 * t43816;
    let t44089 = t3431 * t3431;
    let t44091 = t408 / t44089;
    let t44092 = t3434 * t3434;
    let t44093 = 1.0_f64 / t44092;
    let t44101 = t1126 * t12247;
    let t44125 = t3800 * t3800;
    let t44126 = 1.0_f64 / t44125;
    let t44200 = t12784 * t12773;
    let t44215 = t3625 * t12772 * t12835;
    (t44039, t44040, t44091, t44093, t44101, t44126, t44200, t44215)
}
