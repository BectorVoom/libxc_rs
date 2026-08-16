//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 489/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk489(t1774: f64, t24: f64, t1704: f64, t608: f64, t620: f64, t4834: f64, t1736: f64, t630: f64, t4887: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4893 = t24 * t1774;
    let t4907 = t1704 * t1704;
    let t4908 = 1.0_f64 / t4907;
    let t4909 = t608 * t4908;
    let t4910 = t620 * t620;
    let t4911 = 1.0_f64 / t4910;
    let t4915 = 0.12361111111111111111e-1_f64 * t4834;
    let t4927 = t1736 * t630;
    let t4928 = 1.0_f64 / t4927;
    let t4936 = 0.40256666666666666667e0_f64 * t4834;
    let t4943 = 0.27595e0_f64 * t4887;
    let t4953 = t1736 * t1736;
    let t4954 = 1.0_f64 / t4953;
    (t4893, t4907, t4908, t4909, t4910, t4911, t4915, t4928, t4936, t4943, t4953, t4954)
}
