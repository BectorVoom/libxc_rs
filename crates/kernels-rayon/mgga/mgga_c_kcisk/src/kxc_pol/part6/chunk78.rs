//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 78/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk78(t227: f64, t229: f64, t37: f64, t226: f64, t44: f64, t41: f64, zeta_threshold: f64) -> (f64, f64) {
    let t228 = t227 <= zeta_threshold;
    let t231 = piecewise3(t228, t37, t229 * t227);
    let t233 = (t226 + t231 - 2.0_f64) * t44;
    let t236 = piecewise3(2.0_f64 <= zeta_threshold, t37, 2.0_f64 * t41);
    let t238 = piecewise3(0.0_f64 <= zeta_threshold, t37, 0.0_f64);
    let t240 = (t236 + t238 - 2.0_f64) * t44;
    (t233, t240)
}
