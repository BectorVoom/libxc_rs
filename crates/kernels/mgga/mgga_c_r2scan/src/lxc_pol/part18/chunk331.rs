//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 331/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk331<F: Float>(t54: F, t297: F, t307: F, t23: F, t39: F) -> (F, F, F, F) {
    let t1224 = F::cast_from(1.0_f64) / t54;
    let t1248 = F::cast_from(1.0_f64) / t297;
    let t1256 = F::cast_from(1.0_f64) / t307;
    let t1266 = F::cast_from(1.0_f64) / t23 / t39;
    (t1224, t1248, t1256, t1266)
}
