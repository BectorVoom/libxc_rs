//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1411/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1411<F: Float>(t15125: F, t15168: F, t15191: F, t15197: F, t15127: F, t300: F, t4682: F, t3215: F, t4858: F, t3090: F, t4954: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t15435 = F::cast_from(0.39862222222222222222e0_f64) * t15125;
    let t15447 = F::cast_from(0.21908444444444444444e0_f64) * t15168;
    let t15457 = F::cast_from(0.19931111111111111111e0_f64) * t15191;
    let t15459 = F::cast_from(0.10954222222222222222e0_f64) * t15197;
    let t15483 = F::cast_from(0.41203703703703703704e-2_f64) * t15127;
    let t15484 = F::cast_from(0.12361111111111111111e-1_f64) * t15125;
    let t15485 = F::cast_from(0.61805555555555555556e-2_f64) * t15191;
    let t15503 = F::cast_from(0.23744444444444444444e-1_f64) * t15125;
    let t15504 = F::cast_from(0.11872222222222222222e-1_f64) * t15191;
    let t15547 = t300 * t4682;
    let t15583 = F::cast_from(0.28582678745379824648e-3_f64) * t4858 * t3215;
    let t15618 = t4954 * t3090;
    (t15435, t15447, t15457, t15459, t15483, t15484, t15485, t15503, t15504, t15547, t15583, t15618)
}
