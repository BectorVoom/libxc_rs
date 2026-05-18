//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1118/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1118<F: Float>(t40425: F, t10673: F, t11591: F, t37505: F, t10935: F, t2810: F, t3446: F, t11563: F, t2312: F, t3447: F, t158: F, t2461: F) -> (F, F, F, F, F) {
    let t40426 = F::new(0.72042316457491791906e-3) * t40425;
    let t40428 = t10673 * t11591 * t37505;
    let t40429 = F::new(0.10248087766267884742e-3) * t40428;
    let t40434 = t3446 * t10935 * t2810;
    let t40435 = F::new(0.19211284388664477842e-2) * t40434;
    let t40451 = t3446 * t3447 * t11563 * t2312;
    let t40453 = t158 * t2461;
    (t40426, t40429, t40435, t40451, t40453)
}
