//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1035/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1035<F: Float>(t146: F, t6091: F, t978: F, t2145: F, t2832: F, t537: F, t8691: F, t277: F, t3216: F, t6212: F, t3016: F, t8001: F, t910: F) -> (F, F, F, F, F, F, F) {
    let t26282 = t146 * t6091 * t978;
    let t27067 = t146 * t2145 * t2832;
    let t27661 = t537 * t8691;
    let t27914 = t277 * t8691;
    let t27955 = t6212 * t3216;
    let t27977 = t6212 * t3016;
    let t27996 = t8001 * t910;
    (t26282, t27067, t27661, t27914, t27955, t27977, t27996)
}
