//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 731/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk731(t11514: f64, t254: f64, t2157: f64, t1105: f64, t816: f64, t1109: f64, t346: f64, t3747: f64, t1114: f64, t2319: f64, t3863: f64, t3703: f64, t5: f64, param_a_c: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11539 = t254 * t11514;
    let t11540 = t2157 * param_a_c;
    let t11551 = t816 * t1105;
    let t11557 = t816 * t1109;
    let t11563 = t3747 * t346;
    let t11564 = t1114 * t11563;
    let t11581 = t2319 * t3863;
    let t11583 = t5 * t3703;
    (t11539, t11540, t11551, t11557, t11563, t11564, t11581, t11583)
}
