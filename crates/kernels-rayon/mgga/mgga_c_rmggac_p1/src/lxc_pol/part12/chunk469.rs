//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 469/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk469(t1635: f64, t352: f64, t128: f64, t4928: f64, t326: f64, t3814: f64) -> (f64, f64, f64) {
    let t5156 = t1635 * t352;
    let t5159 = t128 * t4928;
    let t5160 = t326 * t5159;
    let t5162 = t3814 * t128;
    (t5156, t5160, t5162)
}
