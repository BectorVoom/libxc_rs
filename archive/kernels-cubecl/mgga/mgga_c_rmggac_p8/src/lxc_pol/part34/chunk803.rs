//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 803/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk803<F: Float>(t14125: F, t68844: F, t74376: F, t68871: F, t9146: F, t3351: F, t3352: F, t875: F, t8963: F, t1971: F, t7262: F, t8937: F) -> (F, F, F, F) {
    let t74439 = t68844 * t14125 * t74376;
    let t74442 = t68871 * t14125 * t9146;
    let t74446 = t3351 * t3352 * t875 * t8963;
    let t74450 = t3351 * t1971 * t7262 * t8937;
    (t74439, t74442, t74446, t74450)
}
