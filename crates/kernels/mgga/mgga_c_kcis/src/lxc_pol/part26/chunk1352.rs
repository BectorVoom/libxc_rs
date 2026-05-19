//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1352/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1352<F: Float>(t1385: F, t6281: F, t5709: F, t94274: F, t1650: F, t27356: F, t5737: F, t167: F, t16892: F, t1943: F, t101871: F, t101922: F, t101925: F, t103073: F, t20984: F, t21655: F, t27369: F, t27438: F, t27453: F, t27459: F, t29284: F, t3984: F, t59414: F, t7908: F) -> (F, F, F, F) {
    let t103101 = t6281 * t1385;
    let t103103 = t5709 * t94274 * t103101;
    let t103114 = t5709 * t27356 * t1650 * t5737;
    let t103119 = t16892 * t27356 * t167 * t1943;
    let t103132 = F::cast_from(0.27636574074074074073e-2_f64) * t101871 - F::cast_from(0.46336805555555555556e-3_f64) * t7908 * t3984 * t27453 * t59414 - F::cast_from(0.46336805555555555556e-3_f64) * t7908 * t103103 - F::cast_from(0.13901041666666666667e-2_f64) * t7908 * t5709 * t27438 * t20984 + F::cast_from(0.46336805555555555556e-3_f64) * t27459 * t29284 + F::cast_from(0.46336805555555555556e-3_f64) * t7908 * t103114 - F::cast_from(0.92673611111111111112e-3_f64) * t7908 * t103119 - F::cast_from(0.61836467013888888889e-4_f64) * t27369 * t103103 - F::cast_from(0.18534722222222222222e-2_f64) * t7908 * t16892 * t27453 * t21655 - F::cast_from(0.92754700520833333333e-4_f64) * t27369 * t103073 + F::cast_from(0.88437037037037037034e-2_f64) * t101922 - F::cast_from(0.58958024691358024689e-2_f64) * t101925;
    (t103101, t103114, t103119, t103132)
}
