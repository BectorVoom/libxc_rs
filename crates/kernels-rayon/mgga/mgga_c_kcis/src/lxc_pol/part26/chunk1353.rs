//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1353/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1353(t16937: f64, t29357: f64, t7908: f64, t1386: f64, t7086: f64, t101928: f64, t101931: f64, t101934: f64, t101938: f64, t101941: f64, t101948: f64, t101954: f64, t101957: f64, t1380: f64, t28348: f64, t28353: f64, t28372: f64, t98119: f64) -> (f64, f64) {
    let t103141 = t7908 * t16937 * t29357;
    let t103149 = t1386 * t7086;
    let t103154 = 0.17687407407407407407e-1_f64 * t101928 - 0.14739506172839506172e-1_f64 * t101931 + 0.22109259259259259259e-2_f64 * t101934 - 0.66327777777777777776e-2_f64 * t101938 + 0.55273148148148148147e-2_f64 * t101941 - 0.33163888888888888888e-2_f64 * t101948 - 0.7722800925925925926e-4_f64 * t103141 - 0.55652820312500000001e-3_f64 * t98119 * t28353 - 0.18550940104166666667e-3_f64 * t98119 * t28348 - 0.33163888888888888888e-2_f64 * t101954 - 0.11054629629629629629e-2_f64 * t101957 - 0.13901041666666666667e-2_f64 * t7908 * t28372 * t103149 * t1380;
    (t103149, t103154)
}
