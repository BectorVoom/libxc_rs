//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 878/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk878<F: Float>(t7244: F, t8427: F, t2144: F, t3351: F, t352: F, t7231: F, t8502: F, t2001: F, t326: F, t498: F, t559: F, t7720: F, t40948: F, t903: F, t10820: F, t2301: F) -> (F, F, F, F, F) {
    let t42044 = t7244 * t8427;
    let t42050 = t3351 * t7231 * t2144 * t8502 * t352;
    let t42054 = t2001 * t326 * t559 * t498;
    let t42055 = t7720 * t42054;
    let t42057 = t903 * t40948;
    let t42059 = t10820 * t2301;
    (t42044, t42050, t42055, t42057, t42059)
}
