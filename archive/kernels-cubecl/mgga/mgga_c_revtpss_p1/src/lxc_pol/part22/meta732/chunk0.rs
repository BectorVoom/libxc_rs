//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2790/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2790<F: Float>(t212: F, t2237: F, t225: F, t816: F, t2665: F, t10689: F, t237: F, t247: F, t10709: F, t10744: F, t808: F, t2783: F, t9801: F) -> (F, F, F, F, F) {
    let t40488 = t816 * t2237 * t212 * t225;
    let t40489 = t40488 * t2665;
    let t40507 = F::cast_from(0.28974367305964659283e0_f64) * t237 * t10689 * t247;
    let t40509 = t10744 * t808 * t10709;
    let t40517 = t9801 * t2783;
    (t40488, t40489, t40507, t40509, t40517)
}
