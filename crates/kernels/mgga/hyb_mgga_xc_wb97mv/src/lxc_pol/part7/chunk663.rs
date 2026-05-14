//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 663/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk663<F: Float>(t1323: F, t778: F, t238: F, t242: F, t226: F, t3326: F, t2178: F, t2214: F, t2227: F, t2229: F, t3317: F, t3328: F, t3342: F, t3347: F, t3353: F, t3355: F, t3359: F) -> (F, F, F, F, F) {
    let t3361 = t778 * t1323;
    let t3363 = t238 * t242 * t3361;
    let t3365 = t226 * t3326;
    let t3367 = t238 * t242 * t3365;
    let t3369 = -0.9494625e0 * t3342 + 0.1898925e1 * t3347 + t2214 - 0.29896666666666666667e0 * t2178 - 0.29896666666666666667e0 * t3317 + 0.8969e0 * t3328 + 0.15358125e0 * t3353 + 0.3071625e0 * t3355 + t2227 - 0.16431333333333333333e0 * t2229 - 0.16431333333333333333e0 * t3359 + 0.24647e0 * t3363 + 0.24647e0 * t3367;
    (t3361, t3363, t3365, t3367, t3369)
}
