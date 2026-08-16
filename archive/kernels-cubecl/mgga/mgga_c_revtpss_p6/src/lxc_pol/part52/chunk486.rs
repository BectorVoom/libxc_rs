//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 486/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk486<F: Float>(t3434: F, t3356: F, t1156: F, t1160: F, t1159: F, t431: F, t426: F, t3413: F, t434: F, t1175: F, t1179: F, t1178: F, t444: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t3435 = F::cast_from(1.0_f64) / t3434;
    let t3439 = F::cast_from(0.22831111111111111111e-1_f64) * t3356;
    let t3447 = t1156 * t1160;
    let t3450 = t1159 * t431;
    let t3451 = F::cast_from(1.0_f64) / t3450;
    let t3452 = t426 * t3451;
    let t3459 = F::cast_from(0.68863333333333333333e0_f64) * t3356;
    let t3466 = F::cast_from(0.17365833333333333333e0_f64) * t3413;
    let t3475 = t1159 * t1159;
    let t3476 = F::cast_from(1.0_f64) / t3475;
    let t3477 = t426 * t3476;
    let t3478 = t434 * t434;
    let t3479 = F::cast_from(1.0_f64) / t3478;
    let t3483 = F::cast_from(0.12361111111111111111e-1_f64) * t3356;
    let t3491 = t1175 * t1179;
    let t3494 = t1178 * t444;
    (t3435, t3439, t3447, t3452, t3459, t3466, t3477, t3479, t3483, t3491, t3494)
}
