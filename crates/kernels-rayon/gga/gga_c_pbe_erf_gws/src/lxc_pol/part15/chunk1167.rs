//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1167/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1167(t1167: f64, t14149: f64, t944: f64, t3324: f64, t4063: f64, t360: f64, t898: f64, t2416: f64, t2100: f64, t376: f64, t2219: f64, t4383: f64, t4408: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t14829 = t14149 * t1167;
    let t14831 = t1167 * t944;
    let t14835 = t4063 * t3324;
    let t15636 = t898 * t360;
    let t15641 = t2416 * t360;
    let t19615 = t376 * t2100;
    let t19631 = t2219 * t898;
    let t19658 = t4408 * t4383;
    (t14829, t14831, t14835, t15636, t15641, t19615, t19631, t19658)
}
