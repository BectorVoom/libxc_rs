//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 108/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk108<F: Float>(t228: F, t231: F, t234: F, t245: F) -> (F, F, F) {
    let t267 = 0.705945e1 * t231 + 0.1549425e1 * t228 + 0.420775e0 * t234 + 0.1562925e0 * t245;
    let t270 = 1.0 + 0.32163958997385070134e2 / t267;
    let t271 = f64::ln(t270);
    (t267, t270, t271)
}
