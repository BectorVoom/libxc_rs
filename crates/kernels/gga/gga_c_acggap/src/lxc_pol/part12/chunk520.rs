//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 520/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk520<F: Float>(t3125: F, t3126: F, t3124: F, t1049: F, t1056: F, t137: F, t167: F, t130: F, t985: F, t138: F, t1046: F, t134: F) -> (F, F, F, F, F, F) {
    let t3127 = t3125 * t3126;
    let t3128 = t3124 * t3127;
    let t3130 = t1049 * t1056;
    let t3132 = t167 * t137;
    let t3140 = t130 * t985;
    let t3141 = t3140 * t138;
    let t3142 = F::cast_from(70.0_f64) / F::cast_from(27.0_f64) * t3141;
    let t3143 = t1046 * t134;
    (t3128, t3130, t3132, t3141, t3142, t3143)
}
