//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 666/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk666<F: Float>(t3373: F, t808: F, t2245: F, t2178: F, t2251: F, t3317: F, t3328: F, t1345: F, t819: F) -> (F, F, F, F) {
    let t3374 = t3373 * t808;
    let t3376 = 0.16081979498692535067e2 * t2245 * t3374;
    let t3380 = t2251 - 0.17123333333333333333e-1 * t2178 - 0.17123333333333333333e-1 * t3317 + 0.5137e-1 * t3328;
    let t3383 = t1345 * t819;
    (t3374, t3376, t3380, t3383)
}
