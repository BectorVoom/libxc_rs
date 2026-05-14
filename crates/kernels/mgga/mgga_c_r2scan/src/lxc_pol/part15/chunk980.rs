//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 980/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk980<F: Float>(t252: F, t3320: F, t6262: F, t783: F, t10894: F, t1571: F, t3281: F, t6271: F, t10949: F, t10992: F, t2315: F, t3446: F, t158: F, t1783: F, t3447: F, t874: F) -> (F, F, F, F, F, F) {
    let t38189 = t783 * t252 * t6262 * t3320;
    let t38190 = 0.23080304851772712107e1 * t38189;
    let t38191 = t10894 * t1571;
    let t38193 = t3281 * t6271;
    let t38211 = t3446 * t10992 * t10949 * t2315;
    let t38213 = t158 * t1783;
    let t38216 = t3446 * t3447 * t38213 * t874;
    (t38190, t38191, t38193, t38211, t38213, t38216)
}
