//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 497/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk497<F: Float>(t2164: F, t2215: F, t2167: F, t2178: F, t2196: F, t2201: F, t2207: F, t2209: F, t2218: F, t2222: F, t2226: F) -> (F, F, F) {
    let t2297 = F::cast_from(0.40256666666666666667e0_f64) * t2164;
    let t2302 = F::cast_from(0.137975e0_f64) * t2215;
    let t2306 = -F::cast_from(0.1294625e1_f64) * t2196 + F::cast_from(0.258925e1_f64) * t2201 + t2297 - F::cast_from(0.60385e0_f64) * t2167 + F::cast_from(0.905775e0_f64) * t2178 + F::cast_from(0.82524375e-1_f64) * t2207 + F::cast_from(0.16504875e0_f64) * t2209 + t2302 - F::cast_from(0.33114e0_f64) * t2218 + F::cast_from(0.248355e0_f64) * t2222 + F::cast_from(0.248355e0_f64) * t2226;
    (t2297, t2302, t2306)
}
