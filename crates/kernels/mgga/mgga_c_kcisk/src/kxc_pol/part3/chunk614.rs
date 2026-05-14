//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 614/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk614<F: Float>(t10479: F, t5203: F, t1873: F, t1869: F, t5074: F, t5200: F, t227: F, t4596: F) -> (F, F, F) {
    let t10480 = t5203 * t10479;
    let t10481 = t1873 * t10480;
    let t10482 = t1869 * t10481;
    let t10484 = t5074 * t5200;
    let t10487 = 1.0 / t4596 / t227;
    (t10482, t10484, t10487)
}
