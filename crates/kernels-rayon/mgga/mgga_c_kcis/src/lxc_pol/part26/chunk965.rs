//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 965/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk965(t1395: f64, t22224: f64, t1464: f64, t3738: f64, t7262: f64, t3728: f64, t7259: f64, t7198: f64, t12275: f64, t12279: f64, t12303: f64, t12306: f64, t16820: f64, t22215: f64, t22221: f64) -> (f64, f64, f64, f64, f64) {
    let t22225 = t1395 * t22224;
    let t22226 = t1464 * t22225;
    let t22228 = t3738 * t7262;
    let t22229 = t1464 * t22228;
    let t22231 = t3728 * t7259;
    let t22233 = t3728 * t7198;
    let t22235 = -0.36848765432098765431e-3_f64 * t12275 + 0.55273148148148148147e-3_f64 * t12279 - 0.24872916666666666666e-2_f64 * t22215 - 0.55273148148148148147e-3_f64 * t12303 - 0.11054629629629629629e-2_f64 * t16820 + t12306 - 0.24320185185185185185e-1_f64 * t22221 + 0.1621345679012345679e-1_f64 * t22226 - 0.88437037037037037034e-2_f64 * t22229 - 0.16581944444444444444e-2_f64 * t22231 + 0.22109259259259259259e-2_f64 * t22233;
    (t22226, t22229, t22231, t22233, t22235)
}
