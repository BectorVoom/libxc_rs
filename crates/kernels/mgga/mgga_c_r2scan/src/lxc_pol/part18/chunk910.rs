//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 910/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk910<F: Float>(t12422: F, t3271: F, t10924: F, t10933: F, t11589: F, t11593: F, t11604: F, t12406: F, t12410: F, t12413: F, t12417: F, t12420: F, t3229: F, t797: F, t3275: F, t3276: F) -> (F, F, F, F) {
    let t12423 = t12422 * t3271;
    let t12424 = t12423 / 4.0;
    let t12425 = 0.72042316457491791906e-3 * t11589 - 0.10248087766267884742e-3 * t11593 + t12406 - 0.30487649791575028314e-3 * t11604 - t12410 + t12413 - t12417 - t12420 + t10924 - t10933 - t12424;
    let t12428 = t797 * t3229;
    let t12430 = t3275 * t3276 * t12428;
    (t12424, t12425, t12428, t12430)
}
