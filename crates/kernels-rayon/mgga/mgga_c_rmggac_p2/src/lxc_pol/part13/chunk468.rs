//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 468/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk468(t352: f64, t5144: f64, t559: f64, t794: f64, t338: f64, t838: f64, t1635: f64, t128: f64, t4928: f64, t326: f64, t3814: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5149 = t5144 * t352;
    let t5152 = t559 * t794;
    let t5155 = t838 * t338;
    let t5156 = t1635 * t352;
    let t5159 = t128 * t4928;
    let t5160 = t326 * t5159;
    let t5162 = t3814 * t128;
    (t5149, t5152, t5155, t5156, t5160, t5162)
}
