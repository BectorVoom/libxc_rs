//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1758/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1758<F: Float>(t3515: F, t3520: F, t5206: F, t1196: F, t1129: F, t3431: F, t408: F) -> (F, F, F, F) {
    let t12222 = t3520 * t3515 * t5206;
    let t12224 = F::cast_from(0.51947577317044391277e2_f64) * t1196 * t12222;
    let t12226 = F::cast_from(1.0_f64) / t3431 / t1129;
    let t12227 = t408 * t12226;
    (t12222, t12224, t12226, t12227)
}
