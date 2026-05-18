//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1354/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1354<F: Float>(t20956: F, t4261: F, t4260: F, t17391: F, t5913: F, t21799: F, t6028: F, t17508: F, t17504: F, t4122: F, t6012: F, t20934: F, t4293: F) -> (F, F, F, F, F) {
    let t22324 = t4261 * t20956;
    let t22325 = t4260 * t22324;
    let t22327 = t17391 * t5913;
    let t22329 = t6028 * t21799;
    let t22330 = t17508 * t22329;
    let t22332 = t4122 * t17504;
    let t22333 = t22332 * t6012;
    let t22335 = t4293 * t20934;
    (t22325, t22327, t22330, t22333, t22335)
}
