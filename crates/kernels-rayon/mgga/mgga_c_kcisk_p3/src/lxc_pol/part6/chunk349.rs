//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 349/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk349(t385: f64, t338: f64, t2147: f64, t2153: f64, t340: f64, t379: f64, t382: f64, t395: f64, t1313: f64, t2059: f64, t1312: f64, t2110: f64, sigma0: f64) -> (f64, f64, f64, f64, f64) {
    let t386 = t385 < -0.66725e-1_f64;
    let t400 = 0.0_f64 < t338;
    let t2158 = piecewise3(t386, 0.0_f64, 10.0_f64 / 9.0_f64 * t340 * t2147 * t382 - 10.0_f64 / 27.0_f64 * t340 * t379 * t2153);
    let t2159 = t2158 * sigma0;
    let t2160 = t2159 * t395;
    let t2163 = t1313 * t2059;
    let t2164 = t1312 * t2163;
    let t2168 = piecewise3(t400, t2110, -t2110);
    (t2159, t2160, t2163, t2164, t2168)
}
