//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 935/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk935<F: Float>(t16973: F, t17444: F, t5400: F, t639: F, t1652: F, t5406: F, t1898: F, t17420: F, t17425: F, t17430: F, t17432: F, t17434: F, t17436: F, t17439: F, t17443: F) -> (F, F, F, F) {
    let t17448 = F::cast_from(128.0_f64) / F::cast_from(27.0_f64) * t639 * t5400 * t17444 * t16973;
    let t17449 = t5406 * t1652;
    let t17450 = F::cast_from(32.0_f64) / F::cast_from(45.0_f64) * t17449;
    let t17452 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t5406 * t1898;
    let t17453 = -t17420 - t17425 - t17430 + t17432 + t17434 + t17436 + t17439 + t17443 - t17448 + t17450 - t17452;
    (t17448, t17450, t17452, t17453)
}
