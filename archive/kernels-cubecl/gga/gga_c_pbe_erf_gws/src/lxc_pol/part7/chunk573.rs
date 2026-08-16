//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 573/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk573<F: Float>(t43: F, t4360: F, t476: F, t4353: F, t4356: F, t261: F, t52: F, t1413: F, t422: F, zeta_threshold: F) -> (F, F, F, F) {
    let t44 = t43 <= zeta_threshold;
    let t4361 = t476 * t4360;
    let t4364 = piecewise3::<F>(t44, F::cast_from(0.0_f64), F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t4353 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t4356 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t4361);
    let t4366 = F::cast_from(1.0_f64) / t52 / t261;
    let t4367 = t1413 * t422;
    (t4361, t4364, t4366, t4367)
}
