//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 1051/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk1051<F: Float>(t388: F, t575: F, t7933: F, t7934: F, t535: F, t7244: F, t8422: F, t1598: F, t16503: F, t16504: F, t7448: F, t34724: F, t8646: F) -> (F, F, F, F, F) {
    let t41817 = t7933 * t7934 * t388 * t575;
    let t41818 = F::cast_from(0.72042316457491791906e-3_f64) * t41817;
    let t41821 = t7933 * t7934 * t388 * t535;
    let t41822 = F::cast_from(0.72042316457491791906e-3_f64) * t41821;
    let t41828 = t7244 * t8422;
    let t41829 = F::cast_from(0.19863479950205658386e-4_f64) * t41828;
    let t41834 = t16503 * t16504 * t1598 * t7448;
    let t41836 = t34724 * t8646;
    (t41818, t41822, t41829, t41834, t41836)
}
