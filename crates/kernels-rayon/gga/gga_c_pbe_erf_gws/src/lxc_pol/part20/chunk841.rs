//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 841/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk841(t156: f64, t2885: f64, t496: f64, t1243: f64, t2890: f64, t2897: f64, t501: f64, t395: f64, t1552: f64, t978: f64, t1251: f64, t2863: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8146 = t156 * t2885;
    let t8148 = t496 * t8146 / 3.0_f64;
    let t8149 = t2890 * t1243;
    let t8156 = t501 * t2897;
    let t8158 = 0.146904e1_f64 * t8156 * t395;
    let t8159 = t1552 * t978;
    let t8160 = t8159 * t1251;
    let t8197 = t2863 * t1243;
    (t8146, t8148, t8149, t8158, t8160, t8197)
}
