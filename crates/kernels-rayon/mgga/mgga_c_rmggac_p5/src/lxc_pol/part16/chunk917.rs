//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 917/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk917(t1971: f64, t352: f64, t7230: f64, t875: f64, t9843: f64, t8577: f64, t9171: f64, t1910: f64, t1970: f64, t209: f64, t236: f64, t476: f64, t7231: f64) -> (f64, f64, f64) {
    let t45264 = t7230 * t1971 * t875 * t9843 * t352;
    let t45266 = t8577 * t9171;
    let t45272 = t1970 * t7231 * t236 * t1910 * t476 * t209;
    (t45264, t45266, t45272)
}
