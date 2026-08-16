//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 831/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk831<F: Float>(t7204: F, t74960: F, t15128: F, t333: F, t262: F, t7192: F, t15098: F, t321: F, t1326: F, t68815: F, t1322: F, t235: F, t26115: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t74961 = t7204 * t74960;
    let t74963 = t15128 * t333;
    let t74964 = t262 * t74963;
    let t74965 = t7192 * t74964;
    let t74967 = t15098 * t321;
    let t74968 = t1326 * t74967;
    let t74969 = t68815 * t74968;
    let t74973 = t15098 * t333;
    let t74974 = t1326 * t74973;
    let t74975 = t235 * t26115 * t1322 * t74974;
    (t74961, t74963, t74964, t74965, t74967, t74968, t74969, t74973, t74974, t74975)
}
