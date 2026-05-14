//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 422/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk422<F: Float>(t43: F, t1877: F, t574: F, t577: F, t1850: F, t1853: F, t1863: F, t1870: F, t571: F) -> (F, F, F) {
    let t45 = 0.135e1 < t43;
    let t1879 = t574 * t577 * t1877;
    let t1882 = t1850 + t1853 / 81.0 - t571 * t1863 / 81.0 + t571 * t1870 / 27.0 - t571 * t1879 / 54.0;
    let t1883 = piecewise3(t45, t1882, 0.0);
    (t1879, t1882, t1883)
}
