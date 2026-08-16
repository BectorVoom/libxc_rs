//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1114/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1114(t41039: f64, t41042: f64, t3390: f64, t16824: f64, t186: f64, t211: f64, t41046: f64, t41048: f64, t10465: f64, t12509: f64, t5211: f64, t41053: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t47794 = 64.0_f64 / 45.0_f64 * t41039;
    let t47795 = 64.0_f64 / 45.0_f64 * t41042;
    let t47796 = t3390 * t3390;
    let t47800 = 16.0_f64 / 5.0_f64 * t211 * t186 * t16824 * t47796;
    let t47801 = 32.0_f64 / 15.0_f64 * t41046;
    let t47802 = 32.0_f64 / 15.0_f64 * t41048;
    let t47805 = 64.0_f64 / 15.0_f64 * t5211 * t10465 * t12509;
    let t47806 = 32.0_f64 / 45.0_f64 * t41053;
    (t47794, t47795, t47800, t47801, t47802, t47805, t47806)
}
