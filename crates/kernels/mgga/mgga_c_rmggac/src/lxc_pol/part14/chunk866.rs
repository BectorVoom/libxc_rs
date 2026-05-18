//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 866/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk866<F: Float>(t1462: F, t236: F, t321: F, t3352: F, t8517: F, t1243: F, t1475: F, t1970: F, t7231: F, t833: F, t333: F, t511: F, t7230: F, t8829: F) -> (F, F, F, F) {
    let t39079 = t8517 * t3352 * t236 * t1462 * t321;
    let t39084 = t1970 * t7231 * t236 * t1475 * t1243;
    let t39089 = t1970 * t3352 * t236 * t1475 * t833;
    let t39094 = t7230 * t3352 * t511 * t8829 * t333;
    (t39079, t39084, t39089, t39094)
}
