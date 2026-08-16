//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 500/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk500(t3532: f64, t403: f64, t3278: f64, t3952: f64, t1318: f64, t402: f64, t398: f64, t1322: f64, t1310: f64, t1293: f64, t1308: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3953 = t403 * t3532;
    let t3954 = t3953 * t3278;
    let t3955 = t3952 * t3954;
    let t3959 = 1.0_f64 / t1318 / t402;
    let t3960 = t398 * t3959;
    let t3961 = t1322 * t1322;
    let t3962 = t3960 * t3961;
    let t3963 = t1310 * t3962;
    let t3966 = t1293 * t1308;
    (t3953, t3954, t3955, t3959, t3961, t3962, t3963, t3966)
}
