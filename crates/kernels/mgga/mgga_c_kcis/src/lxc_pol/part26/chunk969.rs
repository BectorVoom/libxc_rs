//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 969/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk969<F: Float>(t22276: F, t4160: F, t17261: F, t5667: F, t11862: F, t7101: F, t1650: F, t5627: F, t4163: F, t4162: F, t167: F, t2001: F) -> (F, F, F, F, F, F) {
    let t22277 = t4160 * t22276;
    let t22279 = t17261 * t5667;
    let t22280 = t4160 * t22279;
    let t22282 = t11862 * t7101;
    let t22284 = t1650 * t5627;
    let t22285 = t4163 * t22284;
    let t22286 = t4162 * t22285;
    let t22287 = t4160 * t22286;
    let t22289 = t167 * t2001;
    (t22277, t22280, t22282, t22285, t22287, t22289)
}
