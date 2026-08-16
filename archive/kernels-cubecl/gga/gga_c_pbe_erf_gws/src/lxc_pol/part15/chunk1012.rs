//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1012/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1012<F: Float>(t343: F, t9079: F, t858: F, t867: F, t866: F, t2164: F, t3168: F, t2210: F, t8804: F, t884: F, t8759: F, t2206: F, t3191: F) -> (F, F, F, F, F, F) {
    let t9080 = t9079 * t343;
    let t9082 = t867 * t858 * t9080;
    let t9084 = t866 * t9082 / F::cast_from(96.0_f64);
    let t9086 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t2164 * t3168;
    let t9088 = t2210 * t858 * t8804;
    let t9090 = t884 * t9088 / F::cast_from(8.0_f64);
    let t9092 = t2210 * t858 * t8759;
    let t9094 = t884 * t9092 / F::cast_from(16.0_f64);
    let t9096 = F::cast_from(7.0_f64) / F::cast_from(24.0_f64) * t2206 * t3191;
    (t9080, t9084, t9086, t9090, t9094, t9096)
}
