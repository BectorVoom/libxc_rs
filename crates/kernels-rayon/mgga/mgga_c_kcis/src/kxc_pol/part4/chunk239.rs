//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 239/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk239(t174: f64, t176: f64, t833: f64, t44: f64, t832: f64, t194: f64, t189: f64, t651: f64, t653: f64, t657: f64, t659: f64, t197: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t175 = t174 <= zeta_threshold;
    let t836 = piecewise3(t175, 0.0_f64, 4.0_f64 / 3.0_f64 * t176 * t833);
    let t838 = (t832 + t836) * t44;
    let t843 = t194 * t194;
    let t844 = 1.0_f64 / t843;
    let t845 = t189 * t844;
    let t850 = -0.1176575e1_f64 * t651 - 0.516475e0_f64 * t653 - 0.2103875e0_f64 * t657 - 0.104195e0_f64 * t659;
    let t851 = 1.0_f64 / t197;
    (t838, t843, t844, t845, t850, t851)
}
