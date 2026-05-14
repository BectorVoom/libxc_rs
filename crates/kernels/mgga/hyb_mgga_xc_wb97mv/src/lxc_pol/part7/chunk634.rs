//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 634/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk634<F: Float>(t43: F, t3: F, t574: F, t577: F, t1850: F, t1853: F, t3023: F, t3028: F, t3034: F, t3040: F, t571: F) -> (F, F, F) {
    let t45 = 0.135e1 < t43;
    let t3042 = t574 * t577 * t3;
    let t3045 = t1850 + t1853 / 162.0 + t3023 / 162.0 - t571 * t3028 / 81.0 + t571 * t3034 / 27.0 - t3040 * t3042 / 27.0;
    let t3046 = piecewise3(t45, t3045, 0.0);
    (t3042, t3045, t3046)
}
