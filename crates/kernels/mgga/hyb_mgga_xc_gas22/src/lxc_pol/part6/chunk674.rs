//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 674/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk674<F: Float>(t1359: F, t828: F, t2167: F, t2218: F, t2258: F, t2263: F, t3300: F, t3311: F, t3325: F, t3330: F, t3336: F, t3338: F, t3342: F, t3346: F, t3350: F) -> (F, F) {
    let t3371 = t1359 * t828;
    let t3385 = -F::new(0.17648625e1) * t3325 + F::new(0.3529725e1) * t3330 + t2258 - F::new(0.516475e0) * t2167 - F::new(0.516475e0) * t3300 + F::new(0.1549425e1) * t3311 + F::new(0.31558125e0) * t3336 + F::new(0.6311625e0) * t3338 + t2263 - F::new(0.20839e0) * t2218 - F::new(0.20839e0) * t3342 + F::new(0.312585e0) * t3346 + F::new(0.312585e0) * t3350;
    (t3371, t3385)
}
