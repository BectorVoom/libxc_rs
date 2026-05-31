//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3138/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3138<F: Float>(t16658: F, t44101: F, t12243: F, t16665: F, t16669: F, t44012: F, t3384: F, t3427: F, t5105: F, t12571: F, t5198: F, t1196: F, t12485: F, t3524: F, t5180: F) -> (F, F, F, F, F, F) {
    let t57833 = F::cast_from(0.28947563097646563121e3_f64) * t44101 * t16658;
    let t57835 = F::cast_from(0.48245938496077605201e2_f64) * t12243 * t16665;
    let t57837 = F::cast_from(0.1551780387578202009e4_f64) * t44012 * t16669;
    let t57840 = F::cast_from(6.0_f64) * t3384 * t5105 * t3427;
    let t57842 = F::cast_from(0.35089341735807877242e1_f64) * t12571 * t5198;
    let t57846 = F::cast_from(0.31168546390226634765e3_f64) * t1196 * t12485 * t5180 * t3524;
    (t57833, t57835, t57837, t57840, t57842, t57846)
}
