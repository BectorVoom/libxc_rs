//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 937/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk937<F: Float>(t336: F, t9239: F, t2263: F, t339: F, t824: F, t2262: F, t359: F, t362: F, t366: F, t899: F, t2157: F, t2264: F, t2331: F) -> (F, F, F, F, F) {
    let t20842 = t9239 * t336;
    let t20876 = t339 * t2263;
    let t20877 = t824 * t20876;
    let t20930 = F::cast_from(1.0_f64) / t2262 / t359 * t362;
    let t20932 = t899 * t20930 * t366;
    let t20933 = t2157 * t2157;
    let t20940 = t899 * t2264 * t2331;
    (t20842, t20877, t20932, t20933, t20940)
}
