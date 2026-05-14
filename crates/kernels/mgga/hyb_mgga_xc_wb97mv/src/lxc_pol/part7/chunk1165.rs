//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1165/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1165<F: Float>(t24893: F, t3042: F, t1852: F, t8231: F, t8225: F, t8241: F, t8258: F, t8254: F, t3034: F, t6134: F, t8245: F, t3028: F, t8236: F, t8249: F, t21352: F, t39: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t24894 = t24893 * t3042;
    let t24896 = t1852 * t8231;
    let t24898 = t8225 * t8241;
    let t24900 = t1852 * t8258;
    let t24902 = t8225 * t8254;
    let t24904 = t6134 * t3034;
    let t24906 = t1852 * t8245;
    let t24915 = t6134 * t3028;
    let t24943 = t1852 * t8236;
    let t24945 = t1852 * t8249;
    let t24947 = t21352 * t39;
    (t24894, t24896, t24898, t24900, t24902, t24904, t24906, t24915, t24943, t24945, t24947)
}
