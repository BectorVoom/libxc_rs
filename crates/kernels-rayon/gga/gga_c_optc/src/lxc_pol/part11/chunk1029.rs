//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1029/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1029(t102: f64, t108: f64, t176: f64, t203: f64, t23331: f64, t1974: f64, t6569: f64, t732: f64, t188: f64, t202: f64, t23047: f64, t6602: f64, t740: f64) -> (f64, f64, f64, f64, f64) {
    let t23336 = t176 * t23331 * t102 * t108 * t203 / 2.0_f64;
    let t23392 = t1974 * t1974;
    let t23393 = 1.0_f64 / t23392;
    let t23413 = 1820.0_f64 / 27.0_f64 * t732 * t6569;
    let t23431 = 7280.0_f64 / 81.0_f64 * t188 * t23047 * t202;
    let t23438 = 14.0_f64 / 3.0_f64 * t6602 * t740;
    (t23336, t23393, t23413, t23431, t23438)
}
