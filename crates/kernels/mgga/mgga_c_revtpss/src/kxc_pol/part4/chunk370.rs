//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 370/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk370<F: Float>(t1179: F, t1187: F, t1188: F, t1196: F, t1118: F, t1124: F, t459: F) -> (F, F, F, F, F) {
    let t1198 = t1179 * t1187 * t1188;
    let t1200 = F::new(0.5848223622634646207e0) * t1196 * t1198;
    let t1201 = F::new(0.83333333333333333333e-2) * t1118;
    let t1203 = -t1201 + F::new(0.83333333333333333333e-2) * t1124;
    let t1204 = t1203 * t459;
    (t1198, t1200, t1201, t1203, t1204)
}
