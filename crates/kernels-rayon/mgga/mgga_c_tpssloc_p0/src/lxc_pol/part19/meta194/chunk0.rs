//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 858/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk858(t10195: f64, t4510: f64, t2980: f64, t9288: f64, t977: f64, t9258: f64, t978: f64, t3008: f64, t343: f64, t984: f64, t4546: f64, t271: f64, t2775: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10196 = t4510 * t10195;
    let t10199 = t2980 * t9288;
    let t10200 = t977 * t10199;
    let t10203 = t978 * t9258;
    let t10204 = t977 * t10203;
    let t10208 = t3008 * t984 * t343;
    let t10209 = t4546 * t10208;
    let t10213 = 1.0_f64 / t271 / t2775;
    (t10196, t10199, t10200, t10203, t10204, t10208, t10209, t10213)
}
