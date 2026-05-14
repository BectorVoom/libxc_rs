//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 667/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk667<F: Float>(t1353: F, t827: F, t2178: F, t2229: F, t2269: F, t2274: F, t3317: F, t3328: F, t3342: F, t3347: F, t3353: F, t3355: F, t3359: F, t3363: F, t3367: F) -> (F, F) {
    let t3388 = t1353 * t827;
    let t3402 = -0.17648625e1 * t3342 + 0.3529725e1 * t3347 + t2269 - 0.516475e0 * t2178 - 0.516475e0 * t3317 + 0.1549425e1 * t3328 + 0.31558125e0 * t3353 + 0.6311625e0 * t3355 + t2274 - 0.20839e0 * t2229 - 0.20839e0 * t3359 + 0.312585e0 * t3363 + 0.312585e0 * t3367;
    (t3388, t3402)
}
