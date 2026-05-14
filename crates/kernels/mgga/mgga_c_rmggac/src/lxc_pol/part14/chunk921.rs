//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 921/952 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk921<F: Float>(t1981: F, t3134: F, t41805: F, t8512: F, t1982: F, t7428: F, t8511: F, t16156: F, t9198: F, t388: F, t575: F, t7933: F, t7934: F, t535: F, t7244: F, t8422: F) -> (F, F, F, F, F, F) {
    let t41808 = t8512 * t1981 * t3134 * t41805;
    let t41811 = t8511 * t7428 * t1982;
    let t41812 = 0.19863479950205658386e-4 * t41811;
    let t41813 = t16156 * t9198;
    let t41817 = t7933 * t7934 * t388 * t575;
    let t41818 = 0.72042316457491791906e-3 * t41817;
    let t41821 = t7933 * t7934 * t388 * t535;
    let t41822 = 0.72042316457491791906e-3 * t41821;
    let t41828 = t7244 * t8422;
    (t41808, t41812, t41813, t41818, t41822, t41828)
}
