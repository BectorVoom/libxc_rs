//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 134/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk134<F: Float>(t343: F, t346: F, t349: F, t353: F) -> (F, F, F) {
    let t381 = 0.51785e1 * t346 + 0.905775e0 * t343 + 0.1100325e0 * t349 + 0.1241775e0 * t353;
    let t384 = 1.0 + 0.29608749977793437516e2 / t381;
    let t385 = f64::ln(t384);
    (t381, t384, t385)
}
