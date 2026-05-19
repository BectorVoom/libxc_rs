//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 965/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk965<F: Float>(t1395: F, t22224: F, t1464: F, t3738: F, t7262: F, t3728: F, t7259: F, t7198: F, t12275: F, t12279: F, t12303: F, t12306: F, t16820: F, t22215: F, t22221: F) -> (F, F, F, F, F) {
    let t22225 = t1395 * t22224;
    let t22226 = t1464 * t22225;
    let t22228 = t3738 * t7262;
    let t22229 = t1464 * t22228;
    let t22231 = t3728 * t7259;
    let t22233 = t3728 * t7198;
    let t22235 = -F::cast_from(0.36848765432098765431e-3_f64) * t12275 + F::cast_from(0.55273148148148148147e-3_f64) * t12279 - F::cast_from(0.24872916666666666666e-2_f64) * t22215 - F::cast_from(0.55273148148148148147e-3_f64) * t12303 - F::cast_from(0.11054629629629629629e-2_f64) * t16820 + t12306 - F::cast_from(0.24320185185185185185e-1_f64) * t22221 + F::cast_from(0.1621345679012345679e-1_f64) * t22226 - F::cast_from(0.88437037037037037034e-2_f64) * t22229 - F::cast_from(0.16581944444444444444e-2_f64) * t22231 + F::cast_from(0.22109259259259259259e-2_f64) * t22233;
    (t22226, t22229, t22231, t22233, t22235)
}
