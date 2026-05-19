//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 572/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk572<F: Float>(t305: F, t7779: F, t7769: F, t797: F, t7707: F, t793: F, t128: F, t830: F) -> (F, F, F, F) {
    let t7793 = t305 * t7779;
    let t7795 = t797 * t7769;
    let t7796 = F::cast_from(0.23948483403727617128e0_f64) * t7795;
    let t7815 = t793 * t7707;
    let t7816 = F::cast_from(0.15965655602485078085e0_f64) * t7815;
    let t7817 = t128 * t830;
    (t7793, t7796, t7816, t7817)
}
