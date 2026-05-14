//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 777/951 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk777<F: Float>(t321: F, t3351: F, t515: F, t7248: F, t9049: F, t498: F, t7230: F, t7231: F, t9044: F, t3352: F, t1986: F, t326: F, t495: F, t559: F, t7717: F, t1624: F, t236: F, t9188: F) -> (F, F, F, F, F) {
    let t39127 = t3351 * t7248 * t515 * t9049 * t321;
    let t39132 = t7230 * t7231 * t515 * t9044 * t498;
    let t39137 = t7230 * t3352 * t515 * t9044 * t321;
    let t39141 = t1986 * t326 * t559 * t495;
    let t39142 = t7717 * t39141;
    let t39147 = t7230 * t9188 * t236 * t1624 * t495;
    (t39127, t39132, t39137, t39142, t39147)
}
