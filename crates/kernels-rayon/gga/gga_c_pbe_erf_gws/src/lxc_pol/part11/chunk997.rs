//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 997/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk997(t3916: f64, t6677: f64, t6671: f64, t1114: f64, t6670: f64, t9847: f64, t1109: f64, t2105: f64, t20271: f64, t3765: f64, t1105: f64, t3880: f64, t6228: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t37750 = t3916 * t6677;
    let t37755 = t3916 * t6671;
    let t37768 = t1114 * t9847 * t6670;
    let t37800 = t2105 * t1109;
    let t37814 = t20271 * t3765;
    let t37829 = t2105 * t1105;
    let t37938 = t6228 * t3880;
    (t37750, t37755, t37768, t37800, t37814, t37829, t37938)
}
