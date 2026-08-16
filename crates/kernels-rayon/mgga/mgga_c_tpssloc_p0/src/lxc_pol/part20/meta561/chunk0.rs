//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2117/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2117(t10957: f64, t3053: f64, t271: f64, t2770: f64, t10321: f64, t1041: f64, t248: f64, t3051: f64, t10459: f64, t3117: f64, t10469: f64, t990: f64) -> (f64, f64, f64, f64, f64) {
    let t42303 = t10957 * t3053;
    let t42308 = 1.0_f64 / t271 / t2770;
    let t42322 = t1041 * t248 * t3051 * t10321;
    let t42324 = t3117 * t10459;
    let t42332 = t990 * t10469;
    (t42303, t42308, t42322, t42324, t42332)
}
