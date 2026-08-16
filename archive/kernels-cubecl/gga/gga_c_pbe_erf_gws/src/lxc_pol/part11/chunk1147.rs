//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1147/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1147<F: Float>(t3392: F, t3479: F, t12480: F, t1820: F, t1821: F, t995: F, t3493: F, t3555: F, t11032: F, t3519: F, t3523: F, t12639: F, t2612: F) -> (F, F, F, F, F, F) {
    let t48261 = F::cast_from(8.0_f64) / F::cast_from(5.0_f64) * t3479 * t3392;
    let t48265 = F::cast_from(32.0_f64) / F::cast_from(45.0_f64) * t1820 * t1821 * t12480 * t995;
    let t48267 = F::cast_from(8.0_f64) / F::cast_from(5.0_f64) * t3493 * t3555;
    let t48270 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t11032 * t3519;
    let t48272 = F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t11032 * t3523;
    let t48274 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t2612 * t12639;
    (t48261, t48265, t48267, t48270, t48272, t48274)
}
