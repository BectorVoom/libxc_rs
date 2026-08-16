//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 755/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk755<F: Float>(t3521: F, t820: F, t10469: F, t466: F, t10471: F, t1208: F, t478: F, t10477: F, t483: F, t3508: F, t475: F, t3503: F) -> (F, F, F, F, F, F, F, F) {
    let t11697 = t820 * t3521;
    let t11712 = t466 * t10469;
    let t11713 = t11712 * t10471;
    let t11714 = t1208 * t1208;
    let t11715 = F::cast_from(1.0_f64) / t11714;
    let t11716 = t11715 * t478;
    let t11717 = t483 * t10477;
    let t11718 = t11716 * t11717;
    let t11719 = t11713 * t11718;
    let t11721 = t3508 * t475;
    let t11727 = t3503 * t11717;
    (t11697, t11712, t11713, t11715, t11717, t11719, t11721, t11727)
}
