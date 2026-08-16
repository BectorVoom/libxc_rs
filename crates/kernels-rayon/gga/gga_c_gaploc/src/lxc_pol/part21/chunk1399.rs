//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1399/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1399(t31021: f64, t31024: f64, t31040: f64, t31044: f64, t31045: f64, t31050: f64, t31053: f64, t31056: f64, t34855: f64, t34860: f64, t34863: f64, t34866: f64, t34869: f64, t34874: f64, t34877: f64, t38277: f64, t4820: f64, t6824: f64) -> f64 {
    let t38769 = 0.76685851907841499354e0_f64 * t31021 + t31024 - t34855 + t34860 + t34863 - t34866 - t34869 + t31040 + t31044 - 0.51123901271894332903e1_f64 * t31045 - 0.15889106645266856297e0_f64 * t6824 * t4820 * t38277 - t31050 + t31053 - t31056 - t34874 + t34877;
    t38769
}
