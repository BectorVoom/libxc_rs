//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 973/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk973(t222: f64, t227: f64, t15772: f64, t224: f64, t30153: f64, t30158: f64, t5562: f64, t7710: f64, t15783: f64, t229: f64, t28312: f64, t28368: f64, t5570: f64, t7718: f64, zeta_threshold: f64) -> (f64, f64) {
    let t223 = t222 <= zeta_threshold;
    let t228 = t227 <= zeta_threshold;
    let t30162 = piecewise3(t223, 0.0_f64, -8.0_f64 / 27.0_f64 * t15772 * t30153 + 4.0_f64 / 3.0_f64 * t5562 * t7710 + 4.0_f64 / 3.0_f64 * t224 * t30158);
    let t30170 = piecewise3(t228, 0.0_f64, -8.0_f64 / 27.0_f64 * t15783 * t28368 + 4.0_f64 / 3.0_f64 * t5570 * t7718 + 4.0_f64 / 3.0_f64 * t229 * t28312);
    (t30162, t30170)
}
