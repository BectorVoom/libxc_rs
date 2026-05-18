//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 752/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk752<F: Float>(t2164: F, t7556: F, t7553: F, t7555: F, t288: F, t49: F, t2038: F, t7756: F, t7933: F, t108: F, t4179: F, t490: F) -> (F, F, F, F) {
    let t35244 = t2164 * t7556;
    let t35246 = t7553 * t7555 * t35244;
    let t35253 = t49 * t288;
    let t35256 = t7933 * t2038 * t35253 * t7756;
    let t35311 = t4179 * t108;
    let t35312 = t490 * t35311;
    (t35246, t35253, t35256, t35312)
}
