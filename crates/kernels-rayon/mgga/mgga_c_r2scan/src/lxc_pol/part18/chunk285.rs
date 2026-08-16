//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 285/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk285(t44: f64, t51: f64, t41: f64, t899: f64, t86: f64, t898: f64, t472: f64, t889: f64, t476: f64, t893: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
    let t45 = t44 <= zeta_threshold;
    let t52 = t51 <= zeta_threshold;
    let t900 = t41 * t899;
    let t901 = t898 * t86;
    let t902 = 0.19751673498613801407e-1_f64 * t901;
    let t903 = t472 * t889;
    let t905 = piecewise3(t45, 0.0_f64, 2.0_f64 / 3.0_f64 * t903);
    let t906 = t476 * t893;
    let t908 = piecewise3(t52, 0.0_f64, 2.0_f64 / 3.0_f64 * t906);
    let t910 = t905 / 2.0_f64 + t908 / 2.0_f64;
    (t900, t902, t903, t906, t910)
}
