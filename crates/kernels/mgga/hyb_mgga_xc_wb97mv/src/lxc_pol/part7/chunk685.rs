//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 685/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk685<F: Float>(t1386: F, t929: F, t238: F, t242: F, t341: F, t3487: F, t2453: F, t2489: F, t2499: F, t2501: F, t3478: F, t3489: F, t3503: F, t3508: F, t3514: F, t3516: F, t3520: F) -> (F, F, F, F, F) {
    let t3522 = t929 * t1386;
    let t3524 = t238 * t242 * t3522;
    let t3526 = t341 * t3487;
    let t3528 = t238 * t242 * t3526;
    let t3530 = -0.9494625e0 * t3503 + 0.1898925e1 * t3508 + t2489 - 0.29896666666666666667e0 * t2453 - 0.29896666666666666667e0 * t3478 + 0.8969e0 * t3489 + 0.15358125e0 * t3514 + 0.3071625e0 * t3516 + t2499 - 0.16431333333333333333e0 * t2501 - 0.16431333333333333333e0 * t3520 + 0.24647e0 * t3524 + 0.24647e0 * t3528;
    (t3522, t3524, t3526, t3528, t3530)
}
