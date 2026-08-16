//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1105/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1105<F: Float>(t3139: F, t6646: F, t14101: F, t1176: F, t923: F, t931: F, t3985: F, t376: F, t911: F, t2158: F, t3990: F, t3989: F) -> (F, F, F, F, F, F, F, F) {
    let t14102 = t3139 * t6646;
    let t14103 = t14101 * t14102;
    let t14113 = t1176 * t923 * t931;
    let t14114 = t14113 * t3985;
    let t14115 = F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t14114;
    let t14116 = t911 * t376;
    let t14118 = t3990 * t14116 * t2158;
    let t14119 = t3989 * t14118;
    (t14102, t14103, t14113, t14114, t14115, t14116, t14118, t14119)
}
