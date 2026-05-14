//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 451/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk451<F: Float>(t352: F, t5144: F, t559: F, t794: F, t338: F, t838: F, t1635: F, t128: F, t4928: F, t326: F, t3814: F) -> (F, F, F, F, F, F) {
    let t5149 = t5144 * t352;
    let t5152 = t559 * t794;
    let t5155 = t838 * t338;
    let t5156 = t1635 * t352;
    let t5159 = t128 * t4928;
    let t5160 = t326 * t5159;
    let t5162 = t3814 * t128;
    (t5149, t5152, t5155, t5156, t5160, t5162)
}
