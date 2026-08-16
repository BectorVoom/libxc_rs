//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 740/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk740(t10777: f64, t11252: f64, t11255: f64, t11257: f64, t11261: f64, t11265: f64, t11271: f64, t11275: f64, t11281: f64, t11322: f64, t11392: f64, t11448: f64, t1421: f64, t1689: f64, t4794: f64, t604: f64) -> f64 {
    let t11450 = -12.0_f64 * t1689 * t4794 + t11252 - 4.0_f64 * t604 * t10777 + 0.39422578e-2_f64 * t11255 - 0.26281718666666666667e-2_f64 * t11257 + 0.39422577999999999999e-2_f64 * t1421 * t11261 + 0.39422577999999999999e-2_f64 * t1421 * t11265 + 0.1478346675e-2_f64 * t1421 * t11271 - 0.59133867e-2_f64 * t1421 * t11275 + 0.49278222499999999999e-2_f64 * t1421 * t11281 + t11322 + t11392 + t11448;
    t11450
}
