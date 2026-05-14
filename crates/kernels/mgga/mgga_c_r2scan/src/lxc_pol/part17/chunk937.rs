//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 937/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk937<F: Float>(t545: F, t7600: F, t146: F, t6091: F, t978: F, t2145: F, t2832: F, t537: F, t8691: F, t277: F, t3216: F, t6212: F, t3016: F, t8001: F, t910: F, t2526: F, t2562: F) -> (F, F, F, F, F, F, F, F, F) {
    let t26278 = t545 * t7600;
    let t26282 = t146 * t6091 * t978;
    let t27067 = t146 * t2145 * t2832;
    let t27661 = t537 * t8691;
    let t27914 = t277 * t8691;
    let t27955 = t6212 * t3216;
    let t27977 = t6212 * t3016;
    let t27996 = t8001 * t910;
    let t28000 = t2562 * t2526;
    (t26278, t26282, t27067, t27661, t27914, t27955, t27977, t27996, t28000)
}
