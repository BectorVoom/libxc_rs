//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 923/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk923<F: Float>(t4981: F, t586: F, t593: F, t5357: F, t579: F, t5372: F, t645: F, t1651: F, t5509: F, t587: F, t1648: F, t5413: F) -> (F, F, F, F, F) {
    let t17298 = t4981 * t586;
    let t17300 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t17298 * t593;
    let t17301 = t579 * t5357;
    let t17302 = F::cast_from(64.0_f64) / F::cast_from(405.0_f64) * t17301;
    let t17303 = t5372 * t586;
    let t17305 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t17303 * t645;
    let t17307 = t587 * t1651 * t5509;
    let t17308 = F::cast_from(64.0_f64) / F::cast_from(45.0_f64) * t17307;
    let t17309 = t1648 * t5413;
    (t17300, t17302, t17305, t17308, t17309)
}
