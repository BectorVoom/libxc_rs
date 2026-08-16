//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 864/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk864(t4797: f64, t9429: f64, t1769: f64, t9528: f64, t2861: f64, t5020: f64, t5010: f64, t2825: f64, t5013: f64, t1092: f64, t5014: f64, t10250: f64, t1773: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13301 = t9429 * t4797;
    let t13302 = 0.14739506172839506172e-2_f64 * t13301;
    let t13303 = t9528 * t1769;
    let t13305 = t2861 * t5020;
    let t13307 = t2861 * t5010;
    let t13308 = 0.22109259259259259258e-2_f64 * t13307;
    let t13309 = t2825 * t5013;
    let t13310 = t1092 * t13309;
    let t13312 = t2861 * t5014;
    let t13314 = t10250 * t1773;
    (t13301, t13302, t13303, t13305, t13307, t13308, t13310, t13312, t13314)
}
