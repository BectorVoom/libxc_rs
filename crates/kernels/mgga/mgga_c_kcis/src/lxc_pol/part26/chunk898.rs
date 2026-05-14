//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 898/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk898<F: Float>(t21905: F, t5904: F, t4292: F, t20934: F, t4261: F, t4260: F, t21804: F, t4293: F, t6010: F, t2034: F, t492: F, t5910: F, t16622: F, t4291: F, t6012: F, t17412: F, t5932: F) -> (F, F, F, F, F, F, F, F, F) {
    let t22381 = t5904 * t21905;
    let t22382 = t4292 * t22381;
    let t22384 = t4261 * t20934;
    let t22385 = t4260 * t22384;
    let t22387 = t4293 * t21804;
    let t22388 = t6010 * t22387;
    let t22390 = t2034 * t492;
    let t22391 = t22390 * t5910;
    let t22393 = t16622 * t4291;
    let t22394 = t22393 * t6012;
    let t22396 = t17412 * t5932;
    (t22381, t22382, t22384, t22385, t22387, t22388, t22391, t22394, t22396)
}
