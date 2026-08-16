//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 959/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk959(t1415: f64, t14183: f64, t1411: f64, t1404: f64, t3783: f64, t3787: f64, t3508: f64, t3791: f64, t3513: f64, t3739: f64, t14160: f64, t14162: f64, t14167: f64, t14171: f64, t14173: f64, t14177: f64, t14179: f64, t14181: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14184 = t1415 * t14183;
    let t14185 = t1411 * t14184;
    let t14187 = t1404 * t3783;
    let t14188 = t14187 * sigma0;
    let t14189 = t14188 * t3787;
    let t14190 = t1411 * t14189;
    let t14192 = t3508 * t3791;
    let t14193 = t1411 * t14192;
    let t14195 = t3739 * t3513;
    let t14197 = -0.16581944444444444444e-2_f64 * t14160 + 0.48640370370370370369e-1_f64 * t14162 - 0.11349419753086419753e0_f64 * t14167 + 0.2653111111111111111e-1_f64 * t14171 - 0.2653111111111111111e-1_f64 * t14173 - 0.49745833333333333332e-2_f64 * t14177 - 0.66327777777777777776e-2_f64 * t14179 - 0.49745833333333333332e-2_f64 * t14181 + 0.19898333333333333333e-1_f64 * t14185 + 0.1492375e-1_f64 * t14190 + 0.39796666666666666665e-1_f64 * t14193 + 0.66327777777777777776e-2_f64 * t14195;
    (t14185, t14187, t14190, t14193, t14195, t14197)
}
