//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 702/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk702(t1756: f64, t36: f64, t2079: f64, t262: f64, t5271: f64, t9708: f64, t5259: f64, t9704: f64, t10053: f64, t3814: f64, t645: f64, t9908: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10122 = t36 * t1756;
    let t10124 = t2079 * t262 * t10122;
    let t10135 = t5271 * t9708;
    let t10137 = t5259 * t9704;
    let t10141 = t3814 * t10053;
    let t10151 = t9908 * t645;
    (t10122, t10124, t10135, t10137, t10141, t10151)
}
