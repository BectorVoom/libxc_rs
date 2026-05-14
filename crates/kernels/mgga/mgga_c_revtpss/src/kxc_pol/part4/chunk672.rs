//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 672/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk672<F: Float>(t1169: F, t3471: F, t1159: F, t426: F, t434: F, t3453: F, t3356: F, t3358: F, t3365: F, t3370: F, t3374: F, t448: F, t1175: F, t1179: F, t1178: F, t444: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t3472 = t3471 * t1169;
    let t3475 = t1159 * t1159;
    let t3476 = 1.0 / t3475;
    let t3477 = t426 * t3476;
    let t3478 = t434 * t434;
    let t3479 = 1.0 / t3478;
    let t3480 = t3453 * t3479;
    let t3483 = 0.12361111111111111111e-1 * t3356;
    let t3488 = t3483 - 0.61805555555555555556e-2 * t3358 - 0.61805555555555555555e-2 * t3365 + 0.18541666666666666667e-1 * t3370 + 0.92708333333333333333e-2 * t3374;
    let t3489 = t3488 * t448;
    let t3491 = t1175 * t1179;
    let t3494 = t1178 * t444;
    (t3472, t3475, t3476, t3477, t3478, t3479, t3480, t3483, t3488, t3489, t3491, t3494)
}
