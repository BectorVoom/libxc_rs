//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 463/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk463<F: Float>(t2102: F, t2105: F, t265: F, t254: F, t118: F, t510: F, t116: F) -> (F, F, F, F) {
    let t2106 = t2102 * t265 * t2105;
    let t2108 = F::cast_from(0.63479958930231934629e-2_f64) * t254 * t2106;
    let t2110 = F::new(1.0) / t510 / t118;
    let t2111 = t116 * t2110;
    (t2106, t2108, t2110, t2111)
}
