//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 607/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk607<F: Float>(t421: F, t3356: F, t1156: F, t1160: F, t1159: F, t431: F, t426: F, t3413: F, t434: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t3434 = t421 * t421;
    let t3435 = F::new(1.0) / t3434;
    let t3439 = F::cast_from(0.22831111111111111111e-1_f64) * t3356;
    let t3447 = t1156 * t1160;
    let t3450 = t1159 * t431;
    let t3451 = F::new(1.0) / t3450;
    let t3452 = t426 * t3451;
    let t3459 = F::cast_from(0.68863333333333333333e0_f64) * t3356;
    let t3466 = F::cast_from(0.17365833333333333333e0_f64) * t3413;
    let t3475 = t1159 * t1159;
    let t3476 = F::new(1.0) / t3475;
    let t3477 = t426 * t3476;
    let t3478 = t434 * t434;
    (t3434, t3435, t3439, t3447, t3451, t3452, t3459, t3466, t3475, t3476, t3477, t3478)
}
