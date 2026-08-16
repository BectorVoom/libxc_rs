//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 614/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk614<F: Float>(t43: F, t1336: F, t461: F, t428: F, t726: F, t1402: F, t418: F, t1407: F, t4352: F, t4360: F, t47: F, t728: F, t1412: F, t422: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F) {
    let t44 = t43 <= zeta_threshold;
    let t4753 = t1336 * t461;
    let t4754 = F::cast_from(36.0_f64) * t4753;
    let t4755 = t1336 * t428;
    let t4756 = F::cast_from(36.0_f64) * t4755;
    let t4757 = F::cast_from(1.0_f64) / t726;
    let t4760 = t1402 * t418;
    let t4766 = piecewise3::<F>(t44, F::cast_from(0.0_f64), -F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t4757 * t4352 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t4760 * t1407 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t47 * t4360);
    let t4767 = F::cast_from(1.0_f64) / t728;
    let t4770 = t1412 * t422;
    (t4753, t4754, t4755, t4756, t4757, t4760, t4766, t4767, t4770)
}
