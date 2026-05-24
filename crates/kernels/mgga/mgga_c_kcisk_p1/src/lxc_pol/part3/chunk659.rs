//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 659/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk659<F: Float>(t10476: F, t1799: F, t1894: F, t4972: F, t5203: F, t1873: F, t1869: F, t5074: F, t5200: F, t227: F, t4596: F) -> (F, F, F, F, F) {
    let t10477 = t1799 * t10476;
    let t10479 = t1894 * t4972;
    let t10480 = t5203 * t10479;
    let t10481 = t1873 * t10480;
    let t10482 = t1869 * t10481;
    let t10484 = t5074 * t5200;
    let t10487 = F::new(1.0) / t4596 / t227;
    (t10477, t10479, t10482, t10484, t10487)
}
