//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2381/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2381<F: Float>(t2665: F, t40488: F, t10627: F, t10697: F, t236: F, t807: F, t10689: F, t237: F, t247: F, t10709: F, t10744: F, t808: F) -> (F, F, F, F) {
    let t40489 = t40488 * t2665;
    let t40503 = t807 * t236 * t10697 * t10627;
    let t40507 = F::cast_from(0.28974367305964659283e0_f64) * t237 * t10689 * t247;
    let t40509 = t10744 * t808 * t10709;
    (t40489, t40503, t40507, t40509)
}
