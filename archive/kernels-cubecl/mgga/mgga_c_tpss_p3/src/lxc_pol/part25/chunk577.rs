//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 577/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk577<F: Float>(t1015: F, t3090: F, t242: F, t1125: F, t1014: F, t400: F) -> (F, F, F) {
    let t3091 = t3090 * t1015;
    let t3092 = t242 * t3091;
    let t3093 = t1125 * t3092;
    let t3096 = F::cast_from(1.0_f64) / t400 / t1014;
    (t3092, t3093, t3096)
}
