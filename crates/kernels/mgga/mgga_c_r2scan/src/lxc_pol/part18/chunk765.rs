//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 765/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk765<F: Float>(t166: F, t6880: F, t2320: F, t58: F, t766: F, t2332: F, t287: F, t4881: F, t4886: F, t4896: F, t2850: F, t797: F) -> (F, F, F, F, F, F, F, F) {
    let t6881 = t6880 * t166;
    let t6887 = t2320 * t58;
    let t6888 = t6887 * t766;
    let t6897 = F::cast_from(1.0_f64) / t2332 / t287;
    let t6946 = F::cast_from(12.0_f64) * t4881;
    let t6948 = F::cast_from(80.0_f64) * t4886;
    let t6951 = F::cast_from(32.0_f64) * t4896;
    let t6955 = t2850 * t797;
    (t6881, t6887, t6888, t6897, t6946, t6948, t6951, t6955)
}
