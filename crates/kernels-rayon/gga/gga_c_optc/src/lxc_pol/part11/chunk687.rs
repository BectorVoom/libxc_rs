//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 687/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk687(t6695: f64, t1872: f64, t544: f64, t2204: f64, t732: f64, t43: f64, t97: f64, t50: f64, t99: f64, t1998: f64, t509: f64, t1796: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6696 = 96.0_f64 * t6695;
    let t6709 = 12.0_f64 * t544 * t1872;
    let t6711 = 35.0_f64 / 3.0_f64 * t732 * t2204;
    let t6713 = 1.0_f64 / t97 / t43;
    let t6724 = 1.0_f64 / t99 / t50;
    let t6739 = t509 * t1998;
    let t6741 = 0.16265371324172286321e-1_f64 * t1796 * t6739;
    (t6696, t6709, t6711, t6713, t6724, t6739, t6741)
}
