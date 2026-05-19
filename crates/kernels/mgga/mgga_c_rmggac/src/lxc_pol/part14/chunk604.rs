//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 604/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk604<F: Float>(t7810: F, t793: F, t7444: F, t797: F, t7707: F, t128: F, t830: F) -> (F, F, F, F, F) {
    let t7811 = t793 * t7810;
    let t7813 = t797 * t7444;
    let t7815 = t793 * t7707;
    let t7816 = F::cast_from(0.15965655602485078085e0_f64) * t7815;
    let t7817 = t128 * t830;
    (t7811, t7813, t7815, t7816, t7817)
}
