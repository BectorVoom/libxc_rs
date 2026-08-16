//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 435/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk435(t190: f64, t3232: f64, t207: f64, t1031: f64, t981: f64, t1036: f64, t1032: f64, t1039: f64, t205: f64, t3137: f64, t3139: f64, t1001: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3233 = t3232 * t190;
    let t3234 = t3233 * t207;
    let t3236 = t1031 * t981;
    let t3237 = t3236 * t1036;
    let t3239 = t1032 * t1039;
    let t3241 = t205 * t3137;
    let t3242 = t207 * t3139;
    let t3243 = t3241 * t3242;
    let t3245 = t1039 * t1001;
    (t3233, t3234, t3236, t3237, t3239, t3241, t3242, t3243, t3245)
}
