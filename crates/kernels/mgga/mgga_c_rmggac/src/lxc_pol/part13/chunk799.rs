//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 799/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk799<F: Float>(t212: F, t3076: F, t672: F, t678: F, t7186: F, t7294: F, t7299: F, t7313: F, t7326: F, t7336: F, t7346: F, t7355: F) -> (F, F, F, F, F, F, F, F, F) {
    let t37017 = t672 * t212 * t3076 * t678;
    let t37053 = F::cast_from(0.89430439388620083049e-2_f64) * t7186;
    let t37082 = F::cast_from(0.487802396665200453e-2_f64) * t7294;
    let t37083 = F::cast_from(0.11709622077411463733e-2_f64) * t7299;
    let t37086 = F::cast_from(0.18292589874945016987e-2_f64) * t7313;
    let t37089 = F::cast_from(0.205201155180140685e-5_f64) * t7326;
    let t37096 = F::cast_from(0.91462949374725084936e-3_f64) * t7336;
    let t37099 = F::cast_from(0.91462949374725084936e-3_f64) * t7346;
    let t37100 = F::cast_from(0.26021382394247697185e-3_f64) * t7355;
    (t37017, t37053, t37082, t37083, t37086, t37089, t37096, t37099, t37100)
}
