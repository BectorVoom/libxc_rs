//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 525/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk525<F: Float>(t3553: F, t650: F, t186: F, t211: F, t1033: F, t1046: F, t1024: F, t2741: F, t3345: F) -> (F, F, F, F, F, F) {
    let t3554 = t650 * t3553;
    let t3555 = t186 * t3554;
    let t3557 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t211 * t3555;
    let t3559 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t1033 * t1046;
    let t3561 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t2741 * t1024;
    let t3562 = -t3345;
    (t3554, t3555, t3557, t3559, t3561, t3562)
}
