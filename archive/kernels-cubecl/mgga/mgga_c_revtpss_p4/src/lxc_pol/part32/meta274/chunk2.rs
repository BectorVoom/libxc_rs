//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1160/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1160<F: Float>(t30: F, t525: F, t2: F, t22: F, t33: F, t527: F, t2490: F, t737: F, t2492: F, t744: F) -> (F, F, F, F, F) {
    let t9335 = F::cast_from(1.0_f64) / t525 / t30;
    let t9342 = t2 * t22;
    let t9350 = F::cast_from(1.0_f64) / t527 / t33;
    let t9367 = F::cast_from(1.0_f64) / t2490 / t737;
    let t9368 = t2492 * t744;
    (t9335, t9342, t9350, t9367, t9368)
}
