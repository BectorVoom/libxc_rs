//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 969/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk969(t227: f64, t30125: f64, t564: f64, t2671: f64, t8464: f64, t742: f64, t807: f64, t2361: f64, t28312: f64, t565: f64, t806: f64, t2356: f64, t8476: f64, sigma2: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t228 = t227 <= zeta_threshold;
    let t30126 = t564 * t30125;
    let t30127 = 3.0_f64 / 16.0_f64 * t30126;
    let t30128 = t8464 * t2671;
    let t30129 = 3.0_f64 / 8.0_f64 * t30128;
    let t30130 = 1.0_f64 / t742;
    let t30131 = sigma2 * t30130;
    let t30132 = t30131 * t807;
    let t30133 = 3.0_f64 / 8.0_f64 * t30132;
    let t30134 = t8464 * t2361;
    let t30135 = 3.0_f64 / 8.0_f64 * t30134;
    let t30136 = piecewise3(t228, 0.0_f64, t28312);
    let t30137 = t565 * t30136;
    let t30138 = t30137 * t806;
    let t30139 = t564 * t30138;
    let t30140 = t30139 / 16.0_f64;
    let t30141 = t2356 * t8476;
    (t30127, t30129, t30133, t30135, t30140, t30141)
}
