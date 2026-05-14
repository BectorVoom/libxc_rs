//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 512/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk512<F: Float>(t132: F, t2404: F, t2442: F, t2013: F, t1847: F, t222: F, t341: F, zeta_threshold: F) -> (F, F, F) {
    let t133 = t132 <= zeta_threshold;
    let t2443 = t2404 + t2442;
    let t2445 = piecewise3(t133, 0.0, t2013);
    let t2450 = t222 * t1847 * t341;
    (t2443, t2445, t2450)
}
