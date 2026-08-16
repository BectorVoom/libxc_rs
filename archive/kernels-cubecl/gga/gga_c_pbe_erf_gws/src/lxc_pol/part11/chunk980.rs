//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 980/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk980<F: Float>(t1964: F, t3380: F, t11159: F, t547: F, t5621: F, t985: F, t10020: F, t1396: F, t10016: F, t409: F, t1444: F, t9762: F) -> (F, F, F, F, F, F) {
    let t33426 = t3380 * t1964;
    let t33431 = t11159 * t547;
    let t33446 = t5621 * t985;
    let t33523 = t10020 * t1396;
    let t33527 = t409 * t10016;
    let t33530 = t9762 * t1444;
    (t33426, t33431, t33446, t33523, t33527, t33530)
}
