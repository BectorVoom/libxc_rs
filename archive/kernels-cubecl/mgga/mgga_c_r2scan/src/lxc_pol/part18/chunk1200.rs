//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1200/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1200<F: Float>(t10760: F, t19865: F, t30007: F, t261: F, t3299: F, t9366: F, t3594: F, t39745: F, t2147: F, t28005: F, t11727: F, t11748: F) -> (F, F, F, F, F) {
    let t43327 = t19865 * t10760 * t30007;
    let t43330 = t3299 * t261 * t9366;
    let t43332 = t39745 * t3594;
    let t43335 = t2147 * t10760 * t28005;
    let t43337 = t11748 * t11727;
    (t43327, t43330, t43332, t43335, t43337)
}
