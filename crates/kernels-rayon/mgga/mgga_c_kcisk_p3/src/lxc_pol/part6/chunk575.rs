//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 575/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk575(t385: f64, t4143: f64, t8010: f64, t1284: f64, t7831: f64, t2147: f64, t2153: f64, t340: f64, t379: f64, t382: f64, t8003: f64, t395: f64, t3953: f64, t7706: f64, sigma0: f64) -> (f64, f64, f64, f64, f64) {
    let t386 = t385 < -0.66725e-1_f64;
    let t8011 = t4143 * t8010;
    let t8015 = t1284 * t7831;
    let t8020 = piecewise3(t386, 0.0_f64, 10.0_f64 / 9.0_f64 * t340 * t8003 * t382 - 20.0_f64 / 27.0_f64 * t340 * t2147 * t2153 + 40.0_f64 / 81.0_f64 * t340 * t379 * t8011 - 10.0_f64 / 27.0_f64 * t340 * t379 * t8015);
    let t8021 = t8020 * sigma0;
    let t8022 = t8021 * t395;
    let t8032 = t3953 * t7706;
    (t8011, t8015, t8021, t8022, t8032)
}
