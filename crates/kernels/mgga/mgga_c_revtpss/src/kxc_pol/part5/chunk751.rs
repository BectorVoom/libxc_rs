//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 751/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk751<F: Float>(t3357: F, t3358: F, t5044: F, t5049: F, t5054: F, t5058: F, t422: F, t1130: F, t1719: F, t1151: F, t1733: F, t3379: F) -> (F, F, F, F, F) {
    let t5060 = t3357 - F::cast_from(0.5936111111111111111e-2_f64) * t3358 - F::cast_from(0.5936111111111111111e-2_f64) * t5044 - F::cast_from(0.11872222222222222222e-1_f64) * t5049 + F::cast_from(0.35616666666666666666e-1_f64) * t5054 + F::cast_from(0.17808333333333333333e-1_f64) * t5058;
    let t5062 = F::cast_from(0.621814e-1_f64) * t5060 * t422;
    let t5063 = t1719 * t1130;
    let t5065 = F::cast_from(1.0_f64) * t5063 * t1151;
    let t5067 = F::cast_from(1.0_f64) * t3379 * t1733;
    (t5060, t5062, t5063, t5065, t5067)
}
