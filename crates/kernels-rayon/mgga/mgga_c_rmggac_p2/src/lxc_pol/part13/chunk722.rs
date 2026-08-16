//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 722/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk722(t2144: f64, t899: f64, t507: f64, t7262: f64, t1679: f64, t837: f64, t325: f64, t5011: f64, t117: f64, t1249: f64, t4968: f64, t794: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t26287 = t899 * t2144;
    let t26291 = t507 * t7262;
    let t26346 = t1679 * t837;
    let t26370 = t5011 * t325;
    let t26387 = t1249 * t117;
    let t26490 = t4968 * t325;
    let t26531 = t794 * t325;
    (t26287, t26291, t26346, t26370, t26387, t26490, t26531)
}
