//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 728/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk728<F: Float>(t12722: F, t1821: F, t1820: F, t7580: F, t1033: F, t3555: F, t12709: F, t198: F, t186: F, t561: F, t1019: F, t3399: F, t2790: F, t3451: F, t10326: F, t1006: F, t3445: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t12723 = t1821 * t12722;
    let t12725 = 8.0 / 15.0 * t1820 * t12723;
    let t12726 = 8.0 / 135.0 * t7580;
    let t12728 = 2.0 / 5.0 * t1033 * t3555;
    let t12729 = -t12709;
    let t12730 = t198 * t12729;
    let t12731 = t186 * t12730;
    let t12733 = 4.0 / 15.0 * t561 * t12731;
    let t12735 = 4.0 / 5.0 * t3399 * t1019;
    let t12737 = 4.0 / 5.0 * t2790 * t3451;
    let t12739 = 4.0 / 5.0 * t10326 * t3451;
    let t12741 = 2.0 / 5.0 * t1006 * t3445;
    (t12723, t12725, t12726, t12728, t12729, t12730, t12731, t12733, t12735, t12737, t12739, t12741)
}
