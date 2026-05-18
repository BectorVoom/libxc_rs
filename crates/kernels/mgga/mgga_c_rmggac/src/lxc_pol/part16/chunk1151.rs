//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1151/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1151<F: Float>(t10288: F, t10289: F, t10290: F, t10291: F, t10292: F, t10293: F, t10294: F, t10295: F, t10296: F, t42434: F, t8084: F, t10301: F, t10302: F, t10303: F, t10305: F, t10308: F, t10311: F, t10312: F, t10313: F, t42444: F, t42445: F, t8094: F) -> (F, F) {
    let t49834 = t8084 - t42434 + t10288 - t10289 - t10290 - t10291 - t10292 + t10293 + t10294 - t10295 - t10296;
    let t49837 = t8094 + t42444 - t10301 - t10302 - t10303 - t42445 - t10305 - t10308 - t10311 + t10312 + t10313;
    (t49834, t49837)
}
