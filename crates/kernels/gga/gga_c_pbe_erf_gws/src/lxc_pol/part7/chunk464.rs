//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 464/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk464<F: Float>(t1495: F, t2048: F, t312: F, t944: F, t381: F, t310: F, t311: F, t1: F, t305: F, t152: F, t6: F, t279: F, t837: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t2049 = t1495 + t2048;
    let t2050 = t2049 * t312;
    let t2051 = t944 * t944;
    let t2052 = t381 * t381;
    let t2053 = F::cast_from(1.0_f64) / t2052;
    let t2054 = t2051 * t2053;
    let t2057 = F::cast_from(1.0_f64) / t311 / t310;
    let t2059 = t305 * t2057 * t1;
    let t2060 = t152 * t6;
    let t2062 = t2060 * t837 * t279;
    (t2049, t2050, t2051, t2052, t2053, t2054, t2057, t2059, t2060, t2062)
}
