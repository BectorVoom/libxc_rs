//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 525/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk525<F: Float>(t2481: F, t2492: F, t2486: F, t946: F, t2224: F, t238: F, t351: F, t800: F, t952: F) -> (F, F, F, F, F) {
    let t2493 = t2492 * t2481;
    let t2495 = t946 * t2486;
    let t2498 = t238 * t2224 * t351;
    let t2499 = 0.13692777777777777778e0 * t2498;
    let t2501 = t238 * t800 * t952;
    (t2493, t2495, t2498, t2499, t2501)
}
