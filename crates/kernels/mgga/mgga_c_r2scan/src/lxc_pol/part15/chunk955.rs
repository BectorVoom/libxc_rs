//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 955/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk955<F: Float>(t10751: F, t10808: F, t10859: F, t10911: F, t797: F, t1048: F, t499: F, t3347: F, t498: F) -> (F, F, F, F) {
    let t10913 = t10751 + t10808 + t10859 + t10911;
    let t10914 = t10913 * t797;
    let t10916 = t1048 * t499 * t10914;
    let t10917 = t10916 / F::new(4.0);
    let t10918 = t498 * t3347;
    (t10913, t10914, t10917, t10918)
}
