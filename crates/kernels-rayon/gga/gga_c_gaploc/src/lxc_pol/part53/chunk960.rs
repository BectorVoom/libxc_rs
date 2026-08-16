//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 960/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk960(t2684: f64, t2685: f64, t47143: f64, t12213: f64, t2464: f64, t2465: f64, t13851: f64, t2013: f64, t40986: f64, t40989: f64, t13883: f64, t1991: f64, t590: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t47145 = t2684 * t2685 * t47143;
    let t47149 = t2684 * t2464 * t2465 * t12213;
    let t47151 = t2013 * t13851;
    let t47155 = 0.38342925953920749677e0_f64 * t40986;
    let t47157 = 0.72851559312449424385e1_f64 * t40989;
    let t47160 = 0.51123901271894332902e0_f64 * t1991 * t13883 * t590;
    (t47145, t47149, t47151, t47155, t47157, t47160)
}
