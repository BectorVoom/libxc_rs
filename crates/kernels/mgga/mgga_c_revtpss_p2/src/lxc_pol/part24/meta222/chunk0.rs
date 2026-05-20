//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 973/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk973<F: Float>(t12166: F, t342: F, t11631: F, t12051: F, t1129: F, t3431: F, t408: F, t3434: F, t421: F, t418: F, t240: F, t3698: F) -> (F, F, F, F, F, F, F, F) {
    let t12167 = t342 * t12166;
    let t12168 = t12051 * t11631;
    let t12226 = F::new(1.0) / t3431 / t1129;
    let t12227 = t408 * t12226;
    let t12230 = F::new(1.0) / t3434 / t421;
    let t12247 = F::new(1.0) / t3431 / t418;
    let t12248 = t408 * t12247;
    let t12254 = t240 * t3698;
    (t12167, t12168, t12226, t12227, t12230, t12247, t12248, t12254)
}
