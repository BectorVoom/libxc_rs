//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 799/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk799(t7949: f64, t8340: f64, t8344: f64, t7219: f64, t7223: f64, t7227: f64, t7236: f64, t7241: f64, t7253: f64, t7257: f64, t7261: f64, t8026: f64) -> (f64, f64) {
    let t37047 = 3.0_f64 * t7949;
    let t38187 = 0.68186654135613354322e-2_f64 * t8340;
    let t38188 = 0.72042316457491791906e-3_f64 * t8344;
    let t38189 = t7219 + t7223 + t7227 + t7236 - t7241 + t8026 - t7253 - t7257 - t7261 + t38187 - t38188;
    (t37047, t38189)
}
