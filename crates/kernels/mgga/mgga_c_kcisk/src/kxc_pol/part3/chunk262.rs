//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 262/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk262<F: Float>(t1163: F, t1224: F, t1225: F, t1223: F, t357: F, t346: F, t347: F, t1222: F) -> (F, F, F, F, F, F, F) {
    let t1227 = t1224 * t1225 * t1163;
    let t1229 = -t1223 - F::new(0.17808333333333333333e-1) * t1227;
    let t1232 = t357 * t357;
    let t1233 = F::new(1.0) / t1232;
    let t1234 = t346 * t1233;
    let t1235 = F::new(1.0) / t347;
    let t1237 = -t1222 / F::new(3.0) - t1227 / F::new(3.0);
    (t1227, t1229, t1232, t1233, t1234, t1235, t1237)
}
