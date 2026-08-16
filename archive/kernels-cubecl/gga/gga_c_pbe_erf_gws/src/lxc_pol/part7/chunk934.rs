//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 934/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk934<F: Float>(t1645: F, t5470: F, t1627: F, t5485: F, t1630: F, t5484: F, t639: F, t5399: F, t9: F, t5402: F, t17037: F, t219: F) -> (F, F, F, F, F) {
    let t17434 = F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t5470 * t1645;
    let t17436 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t1627 * t5485;
    let t17438 = t639 * t1630 * t5484;
    let t17439 = F::cast_from(32.0_f64) / F::cast_from(135.0_f64) * t17438;
    let t17440 = t9 * t5399;
    let t17442 = t639 * t17440 * t5402;
    let t17443 = F::cast_from(256.0_f64) / F::cast_from(243.0_f64) * t17442;
    let t17444 = t219 * t17037;
    (t17434, t17436, t17439, t17443, t17444)
}
