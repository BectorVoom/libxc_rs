//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 977/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk977(t22479: f64, t5: f64, t21843: f64, t2253: f64, t21847: f64, t21837: f64, t2938: f64, t21856: f64, t21893: f64, t21867: f64, t668: f64, t1268: f64, t4635: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t82074 = t5 * t22479;
    let t82077 = t2253 * t21843;
    let t82079 = t2253 * t21847;
    let t82082 = t2938 * t21837;
    let t82088 = t2253 * t21856;
    let t82095 = t2253 * t21893;
    let t82097 = t2253 * t21867;
    let t82112 = t21837 * t668;
    let t82182 = t4635 * t1268;
    (t82074, t82077, t82079, t82082, t82088, t82095, t82097, t82112, t82182)
}
