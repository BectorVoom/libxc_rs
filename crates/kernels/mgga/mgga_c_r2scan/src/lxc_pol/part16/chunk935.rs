//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 935/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk935<F: Float>(t2574: F, t3433: F, t146: F, t20946: F, t252: F, t545: F, t7600: F, t6091: F, t978: F, t2145: F, t2832: F, t537: F, t8691: F, t277: F, t3216: F, t6212: F) -> (F, F, F, F, F, F, F, F) {
    let t26176 = t3433 * t2574;
    let t26185 = t146 * t20946 * t252;
    let t26278 = t545 * t7600;
    let t26282 = t146 * t6091 * t978;
    let t27067 = t146 * t2145 * t2832;
    let t27661 = t537 * t8691;
    let t27914 = t277 * t8691;
    let t27955 = t6212 * t3216;
    (t26176, t26185, t26278, t26282, t27067, t27661, t27914, t27955)
}
