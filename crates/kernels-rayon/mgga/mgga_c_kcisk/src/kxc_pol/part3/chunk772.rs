//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 772/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk772(t10593: f64, t7378: f64, t140: f64, t446: f64, t728: f64, t299: f64, t5268: f64, t1925: f64, t430: f64, t11638: f64, t673: f64, t1909: f64, t574: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11880 = t7378 * t10593;
    let t11885 = 0.11791604938271604938e-1_f64 * t140 * t446 * t728;
    let t11891 = t140 * t299 * t5268;
    let t11894 = t140 * t430 * t1925;
    let t11896 = t673 * t11638;
    let t11900 = t1909 * t574;
    (t11880, t11885, t11891, t11894, t11896, t11900)
}
