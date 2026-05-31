//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 669/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk669<F: Float>(t5335: F, t561: F, t1879: F, t1882: F, t1735: F, t2730: F, t1748: F, t202: F, t184: F, t619: F, t1871: F, t582: F) -> (F, F, F, F, F, F, F) {
    let t5337 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t561 * t5335;
    let t5338 = t1879 * t1882;
    let t5339 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t5338;
    let t5341 = F::cast_from(4.0_f64) / F::cast_from(5.0_f64) * t2730 * t1735;
    let t5342 = t202 * t1748;
    let t5343 = t5342 * t184;
    let t5345 = F::cast_from(4.0_f64) / F::cast_from(5.0_f64) * t5343 * t619;
    let t5346 = t582 * t1871;
    (t5337, t5339, t5341, t5342, t5343, t5345, t5346)
}
