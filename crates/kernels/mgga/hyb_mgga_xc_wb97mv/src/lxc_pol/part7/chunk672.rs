//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 672/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk672<F: Float>(t1365: F, t846: F, t2178: F, t2229: F, t2308: F, t2313: F, t3317: F, t3328: F, t3342: F, t3347: F, t3353: F, t3355: F, t3359: F, t3363: F, t3367: F) -> (F, F) {
    let t3421 = t1365 * t846;
    let t3435 = -0.1294625e1 * t3342 + 0.258925e1 * t3347 + t2308 - 0.301925e0 * t2178 - 0.301925e0 * t3317 + 0.905775e0 * t3328 + 0.82524375e-1 * t3353 + 0.16504875e0 * t3355 + t2313 - 0.16557e0 * t2229 - 0.16557e0 * t3359 + 0.248355e0 * t3363 + 0.248355e0 * t3367;
    (t3421, t3435)
}
