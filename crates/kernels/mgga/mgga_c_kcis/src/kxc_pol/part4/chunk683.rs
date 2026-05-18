//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 683/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk683<F: Float>(t1335: F, t3893: F, t1316: F, t1314: F, t455: F) -> (F, F, F, F, F) {
    let t3894 = t3893 * t1335;
    let t3896 = F::new(1.0) * t1316 * t3894;
    let t3897 = t1314 * t1314;
    let t3898 = F::new(1.0) / t3897;
    let t3899 = t455 * t3898;
    (t3894, t3896, t3897, t3898, t3899)
}
