//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 497/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk497(t3933: f64, t3934: f64, t1311: f64, t9: f64, t1319: f64, t1163: f64, t1322: f64, t1390: f64, t403: f64, t3278: f64, t1312: f64, t1313: f64, t3283: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3935 = t3933 * t3934;
    let t3936 = t9 * t1311;
    let t3937 = t3936 * t1319;
    let t3938 = t1163 * t1322;
    let t3939 = t3937 * t3938;
    let t3942 = t403 * t1390;
    let t3943 = t3942 * t3278;
    let t3944 = t1312 * t3943;
    let t3947 = t1313 * t3283;
    (t3935, t3936, t3937, t3938, t3939, t3943, t3944, t3947)
}
