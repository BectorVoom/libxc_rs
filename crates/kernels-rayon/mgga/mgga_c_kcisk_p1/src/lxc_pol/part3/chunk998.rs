//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 998/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk998(t13962: f64, t14154: f64, t14157: f64, t14160: f64, t14162: f64, t14167: f64, t14171: f64, t14173: f64, t14177: f64, t14179: f64, t14181: f64, t14185: f64) -> f64 {
    let t14701 = 0.34822083333333333333e-2_f64 * t13962 + 0.17411041666666666666e-2_f64 * t14154 - 0.52233124999999999998e-2_f64 * t14157 - 0.11607361111111111111e-2_f64 * t14160 + 0.34048259259259259259e-1_f64 * t14162 - 0.79445938271604938269e-1_f64 * t14167 + 0.18571777777777777778e-1_f64 * t14171 - 0.18571777777777777778e-1_f64 * t14173 - 0.34822083333333333333e-2_f64 * t14177 - 0.46429444444444444443e-2_f64 * t14179 - 0.34822083333333333333e-2_f64 * t14181 + 0.13928833333333333333e-1_f64 * t14185;
    t14701
}
