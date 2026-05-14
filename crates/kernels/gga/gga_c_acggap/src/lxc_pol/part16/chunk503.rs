//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 503/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk503<F: Float>(t21: F, t5: F, t3125: F, t3124: F, t137: F, t167: F, t130: F, t985: F, t138: F, t1046: F, t134: F, t347: F, t227: F, t8: F, t14: F, t2: F, t41: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t3126 = t21 * t5;
    let t3127 = t3125 * t3126;
    let t3128 = t3124 * t3127;
    let t3132 = t167 * t137;
    let t3140 = t130 * t985;
    let t3141 = t3140 * t138;
    let t3142 = 70.0 / 27.0 * t3141;
    let t3143 = t1046 * t134;
    let t3144 = t3143 * t347;
    let t3151 = 1.0 / t8 / t227;
    let t3152 = t130 * t3151;
    let t3153 = t3152 * t134;
    let t3157 = 1.0 / t14 / t2 / t41 / 48.0;
    (t3126, t3128, t3132, t3141, t3142, t3143, t3144, t3151, t3153, t3157)
}
