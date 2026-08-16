//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 486/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk486<F: Float>(t2252: F, t829: F, t2164: F, t2215: F, t2167: F, t2178: F, t2196: F, t2201: F, t2207: F, t2209: F, t2218: F, t2222: F, t2226: F) -> (F, F, F, F) {
    let t2253 = t2252 * t829;
    let t2258 = F::cast_from(0.68863333333333333333e0_f64) * t2164;
    let t2263 = F::cast_from(0.17365833333333333333e0_f64) * t2215;
    let t2267 = -F::cast_from(0.17648625e1_f64) * t2196 + F::cast_from(0.3529725e1_f64) * t2201 + t2258 - F::cast_from(0.103295e1_f64) * t2167 + F::cast_from(0.1549425e1_f64) * t2178 + F::cast_from(0.31558125e0_f64) * t2207 + F::cast_from(0.6311625e0_f64) * t2209 + t2263 - F::cast_from(0.41678e0_f64) * t2218 + F::cast_from(0.312585e0_f64) * t2222 + F::cast_from(0.312585e0_f64) * t2226;
    (t2253, t2258, t2263, t2267)
}
