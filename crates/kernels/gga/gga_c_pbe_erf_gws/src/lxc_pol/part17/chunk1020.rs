//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1020/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1020<F: Float>(t9182: F, t1123: F, t6491: F, t850: F, t860: F, t2145: F, t3039: F, t2150: F, t3180: F, t6322: F, t3131: F, t6523: F, t6524: F) -> (F, F, F, F, F, F) {
    let t9183 = F::cast_from(35.0_f64) / F::cast_from(432.0_f64) * t9182;
    let t9185 = t850 * t1123 * t6491;
    let t9187 = t9185 * t860 / F::cast_from(96.0_f64);
    let t9188 = t3039 * t2145;
    let t9190 = t9188 * t2150 / F::cast_from(24.0_f64);
    let t9192 = t6322 * t3180 / F::cast_from(48.0_f64);
    let t9194 = t6523 * t3131 * t6524;
    (t9183, t9185, t9187, t9190, t9192, t9194)
}
