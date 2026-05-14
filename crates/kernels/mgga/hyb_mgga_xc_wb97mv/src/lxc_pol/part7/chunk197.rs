//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 197/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk197<F: Float>(t43: F, t544: F, t574: F, t577: F, t569: F, t571: F) -> (F, F, F) {
    let t45 = 0.135e1 < t43;
    let t579 = t574 * t577 * t544;
    let t582 = -t571 * t579 / 54.0 - t569 / 54.0;
    let t583 = piecewise3(t45, t582, 0.0);
    (t579, t582, t583)
}
