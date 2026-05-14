//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 130/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk130<F: Float>(t238: F, t242: F, t351: F, t343: F, t346: F, t349: F) -> (F, F, F, F) {
    let t353 = t238 * t242 * t351;
    let t355 = 0.379785e1 * t346 + 0.8969e0 * t343 + 0.204775e0 * t349 + 0.123235e0 * t353;
    let t358 = 1.0 + 0.16081979498692535067e2 / t355;
    let t359 = f64::ln(t358);
    (t353, t355, t358, t359)
}
