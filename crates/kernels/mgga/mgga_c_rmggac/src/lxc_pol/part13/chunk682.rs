//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 682/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk682<F: Float>(t34884: F, t7239: F, t16156: F, t7746: F, t1990: F, t34881: F, t7234: F, t2185: F, t7690: F, t1997: F, t7414: F, t7696: F, t7939: F, t2186: F, t7682: F, t7905: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t34885 = t34884 * t7239;
    let t34887 = t16156 * t7746;
    let t34889 = t34881 * t1990;
    let t34894 = t34884 * t7234;
    let t34902 = t7690 * t2185;
    let t34903 = t34902 * t1997;
    let t34905 = t7414 * t7696;
    let t34907 = t7939 * t1990;
    let t34911 = t2186 * t7682;
    let t34913 = t2186 * t7905;
    (t34885, t34887, t34889, t34894, t34902, t34903, t34905, t34907, t34911, t34913)
}
