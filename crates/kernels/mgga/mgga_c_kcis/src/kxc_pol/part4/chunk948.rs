//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 948/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk948<F: Float>(t60: F, t9060: F, t9184: F, t20: F, t2394: F, t63: F, t697: F, t2404: F, t700: F, t209: F, t2403: F, t2410: F, t8747: F) -> (F, F, F, F, F) {
    let t70 = F::cast_from(0.0_f64) < t60;
    let t9185 = t9060 + t9184;
    let t9194 = t2394 * t20;
    let t9195 = t63 * t9194;
    let t9202 = t697 * t697;
    let t9203 = F::cast_from(1.0_f64) / t9202;
    let t9204 = t2404 * t700;
    let t9206 = t209 * t9203 * t9204;
    let t9209 = t2403 * t700;
    let t9211 = t209 * t9209 * t2410;
    let t9215 = piecewise3::<F>(t70, t8747, -t8747);
    (t9185, t9195, t9206, t9211, t9215)
}
