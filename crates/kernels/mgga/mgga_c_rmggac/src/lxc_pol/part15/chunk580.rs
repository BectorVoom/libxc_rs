//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 580/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk580<F: Float>(t2127: F, t290: F, t236: F, t830: F, t507: F, t2004: F, t2186: F, t2007: F, t1223: F, t28: F, t212: F, t672: F) -> (F, F, F, F, F, F, F) {
    let t7894 = t290 * t2127;
    let t7900 = t236 * t830;
    let t7901 = t507 * t7900;
    let t7908 = t2186 * t2004;
    let t7909 = F::new(0.19863479950205658386e-4) * t7908;
    let t7910 = t2186 * t2007;
    let t7919 = t1223 * t28;
    let t7920 = t212 * t7919;
    let t7921 = t672 * t7920;
    (t7894, t7900, t7901, t7909, t7910, t7920, t7921)
}
