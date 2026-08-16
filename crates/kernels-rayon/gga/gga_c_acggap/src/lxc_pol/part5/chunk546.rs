//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 546/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk546(t3143: f64, t347: f64, t1049: f64, t1065: f64, t227: f64, t8: f64, t130: f64, t134: f64, t14: f64, t2: f64, t41: f64, t135: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3144 = t3143 * t347;
    let t3146 = t1049 * t1065;
    let t3151 = 1.0_f64 / t8 / t227;
    let t3152 = t130 * t3151;
    let t3153 = t3152 * t134;
    let t3157 = 1.0_f64 / t14 / t2 / t41 / 48.0_f64;
    let t3159 = t135 * t3157 * t2;
    (t3144, t3146, t3151, t3152, t3153, t3157, t3159)
}
