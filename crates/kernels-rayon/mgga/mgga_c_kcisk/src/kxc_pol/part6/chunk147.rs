//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 147/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk147(t227: f64, tau1: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t228 = t227 <= zeta_threshold;
    let t565 = 1.0_f64 / tau1;
    let t566 = piecewise3(t228, zeta_threshold, t227);
    let t567 = t565 * t566;
    let t568 = 1.0_f64 / t227;
    (t565, t567, t568)
}
