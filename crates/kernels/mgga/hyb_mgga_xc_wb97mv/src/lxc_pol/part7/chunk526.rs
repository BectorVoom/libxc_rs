//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 526/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk526<F: Float>(t929: F, t238: F, t242: F, t2462: F, t341: F, t2453: F, t2464: F, t2482: F, t2487: F, t2489: F, t2493: F, t2495: F, t2499: F, t2501: F) -> (F, F, F, F, F) {
    let t2503 = t929 * t929;
    let t2505 = t238 * t242 * t2503;
    let t2507 = t341 * t2462;
    let t2509 = t238 * t242 * t2507;
    let t2511 = -0.9494625e0 * t2482 + 0.1898925e1 * t2487 + t2489 - 0.59793333333333333334e0 * t2453 + 0.8969e0 * t2464 + 0.15358125e0 * t2493 + 0.3071625e0 * t2495 + t2499 - 0.32862666666666666666e0 * t2501 + 0.24647e0 * t2505 + 0.24647e0 * t2509;
    (t2503, t2505, t2507, t2509, t2511)
}
