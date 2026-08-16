//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 613/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk613<F: Float>(t289: F, t7894: F, t739: F, t7855: F, t236: F, t830: F, t507: F, t2007: F, t2191: F, t1260: F, t1986: F, t675: F) -> (F, F, F, F, F, F, F) {
    let t7895 = t289 * t7894;
    let t7896 = F::cast_from(0.4726e1_f64) * t7895;
    let t7897 = t739 * t7855;
    let t7898 = F::cast_from(0.14967802127329760705e-1_f64) * t7897;
    let t7900 = t236 * t830;
    let t7901 = t507 * t7900;
    let t7903 = t2191 * t2007;
    let t7904 = F::cast_from(0.25538759935978703638e-4_f64) * t7903;
    let t7905 = t1986 * t1260;
    let t7906 = t675 * t7905;
    (t7896, t7898, t7900, t7901, t7904, t7905, t7906)
}
