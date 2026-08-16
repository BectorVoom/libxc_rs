//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 929/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk929(t2897: f64, t501: f64, t395: f64, t1552: f64, t978: f64, t1251: f64, t1563: f64, t2873: f64, t102: f64, t2885: f64, t481: f64, t1533: f64, t974: f64) -> (f64, f64, f64, f64, f64) {
    let t8156 = t501 * t2897;
    let t8158 = 0.146904e1_f64 * t8156 * t395;
    let t8159 = t1552 * t978;
    let t8160 = t8159 * t1251;
    let t8162 = t1563 * t2873;
    let t8171 = 0.116921e2_f64 * t102 * t2885 * t481;
    let t8174 = 0.584605e1_f64 * t102 * t974 * t1533;
    (t8158, t8160, t8162, t8171, t8174)
}
