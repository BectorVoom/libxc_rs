//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 477/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk477<F: Float>(t779: F, t238: F, t242: F, t2176: F, t226: F, t2167: F, t2178: F, t2196: F, t2201: F, t2203: F, t2207: F, t2209: F, t2216: F, t2218: F) -> (F, F, F, F, F) {
    let t2220 = t779 * t779;
    let t2222 = t238 * t242 * t2220;
    let t2224 = t226 * t2176;
    let t2226 = t238 * t242 * t2224;
    let t2228 = -F::new(0.9494625e0) * t2196 + F::new(0.1898925e1) * t2201 + t2203 - F::new(0.59793333333333333334e0) * t2167 + F::new(0.8969e0) * t2178 + F::new(0.15358125e0) * t2207 + F::new(0.3071625e0) * t2209 + t2216 - F::new(0.32862666666666666666e0) * t2218 + F::new(0.24647e0) * t2222 + F::new(0.24647e0) * t2226;
    (t2220, t2222, t2224, t2226, t2228)
}
