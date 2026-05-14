//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1205/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1205<F: Float>(t1555: F, t7397: F, t4189: F, t1529: F, t7386: F, t1543: F, t7329: F, t20956: F, t4261: F, t4260: F, t17391: F, t5913: F, t21799: F, t6028: F, t17508: F, t17504: F, t4122: F) -> (F, F, F, F, F, F, F) {
    let t22317 = t7397 * t1555;
    let t22319 = 2.0 * t4189 * t22317;
    let t22320 = t1529 * t7386;
    let t22322 = t1543 * t7329;
    let t22324 = t4261 * t20956;
    let t22325 = t4260 * t22324;
    let t22327 = t17391 * t5913;
    let t22329 = t6028 * t21799;
    let t22330 = t17508 * t22329;
    let t22332 = t4122 * t17504;
    (t22319, t22320, t22322, t22325, t22327, t22330, t22332)
}
