//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1022/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1022(t12468: f64, t587: f64, t7669: f64, t12800: f64, t1820: f64, t12729: f64, t572: f64, t12484: f64, t172: f64, t184: f64, t10300: f64, t2612: f64) -> (f64, f64, f64, f64, f64) {
    let t41769 = t587 * t7669 * t12468;
    let t41772 = t1820 * t7669 * t12800;
    let t41787 = t12729 * t572;
    let t41840 = t172 * t12484 * t184;
    let t41847 = t2612 * t10300;
    (t41769, t41772, t41787, t41840, t41847)
}
